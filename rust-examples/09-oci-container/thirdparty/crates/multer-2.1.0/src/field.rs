use std::borrow::Cow;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut};
use encoding_rs::{Encoding, UTF_8};
use futures_util::stream::{Stream, TryStreamExt};
use http::header::HeaderMap;
#[cfg(feature = "json")]
use serde::de::DeserializeOwned;
use spin::mutex::spin::SpinMutex as Mutex;

use crate::content_disposition::ContentDisposition;
use crate::multipart::{MultipartState, StreamingStage};
use crate::{helpers, Error};

/// A single field in a multipart stream.
///
/// Its content can be accessed via the [`Stream`] API or the methods defined in
/// this type.
///
/// # Lifetime
///
/// The lifetime of the stream `'r` corresponds to the lifetime of the
/// underlying `Stream`. If the underlying stream holds no references directly
/// or transitively, then the lifetime can be `'static`.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
///
/// use bytes::Bytes;
/// use futures_util::stream::once;
/// use multer::Multipart;
///
/// # async fn run() {
/// let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; \
///     name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
///
/// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
/// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
///
/// while let Some(field) = multipart.next_field().await.unwrap() {
///     let content = field.text().await.unwrap();
///     assert_eq!(content, "abcd");
/// }
/// # }
/// # tokio::runtime::Runtime::new().unwrap().block_on(run());
/// ```
///
/// [`Multipart`]: crate::Multipart
#[derive(Debug)]
pub struct Field<'r> {
    state: Arc<Mutex<MultipartState<'r>>>,
    done: bool,
    headers: HeaderMap,
    content_disposition: ContentDisposition,
    content_type: Option<mime::Mime>,
    idx: usize,
}

impl<'r> Field<'r> {
    pub(crate) fn new(
        state: Arc<Mutex<MultipartState<'r>>>,
        headers: HeaderMap,
        idx: usize,
        content_disposition: ContentDisposition,
    ) -> Self {
        let content_type = helpers::parse_content_type(&headers);
        Field {
            state,
            headers,
            content_disposition,
            content_type,
            idx,
            done: false,
        }
    }

    /// The field name found in the [`Content-Disposition`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition) header.
    pub fn name(&self) -> Option<&str> {
        self.content_disposition.field_name.as_deref()
    }

    /// The file name found in the [`Content-Disposition`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition) header.
    pub fn file_name(&self) -> Option<&str> {
        self.content_disposition.file_name.as_deref()
    }

    /// Get the content type of the field.
    pub fn content_type(&self) -> Option<&mime::Mime> {
        self.content_type.as_ref()
    }

    /// Get a map of headers as [`HeaderMap`].
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get the full data of the field as [`Bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use bytes::Bytes;
    /// use futures_util::stream::once;
    /// use multer::Multipart;
    ///
    /// # async fn run() {
    /// let data =
    ///     "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
    /// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    /// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    ///
    /// while let Some(field) = multipart.next_field().await.unwrap() {
    ///     let bytes = field.bytes().await.unwrap();
    ///     assert_eq!(bytes.len(), 4);
    /// }
    /// # }
    /// # tokio::runtime::Runtime::new().unwrap().block_on(run());
    /// ```
    pub async fn bytes(self) -> crate::Result<Bytes> {
        let mut buf = BytesMut::new();

        let mut this = self;
        while let Some(bytes) = this.chunk().await? {
            buf.extend_from_slice(&bytes);
        }

        Ok(buf.freeze())
    }

    /// Stream a chunk of the field data.
    ///
    /// When the field data has been exhausted, this will return [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use bytes::Bytes;
    /// use futures_util::stream::once;
    /// use multer::Multipart;
    ///
    /// # async fn run() {
    /// let data =
    ///     "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
    /// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    /// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    ///
    /// while let Some(mut field) = multipart.next_field().await.unwrap() {
    ///     while let Some(chunk) = field.chunk().await.unwrap() {
    ///         println!("Chunk: {:?}", chunk);
    ///     }
    /// }
    /// # }
    /// # tokio::runtime::Runtime::new().unwrap().block_on(run());
    /// ```
    pub async fn chunk(&mut self) -> crate::Result<Option<Bytes>> {
        self.try_next().await
    }

    /// Try to deserialize the field data as JSON.
    ///
    /// # Optional
    ///
    /// This requires the optional `json` feature to be enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use multer::Multipart;
    /// use bytes::Bytes;
    /// use std::convert::Infallible;
    /// use futures_util::stream::once;
    /// use serde::Deserialize;
    ///
    /// // This `derive` requires the `serde` dependency.
    /// #[derive(Deserialize)]
    /// struct User {
    ///     name: String
    /// }
    ///
    /// # async fn run() {
    /// let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\n{ \"name\": \"Alice\" }\r\n--X-BOUNDARY--\r\n";
    /// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    /// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    ///
    /// while let Some(field) = multipart.next_field().await.unwrap() {
    ///     let user = field.json::<User>().await.unwrap();
    ///     println!("User Name: {}", user.name);
    /// }
    /// # }
    /// # tokio::runtime::Runtime::new().unwrap().block_on(run());
    /// ```
    ///
    /// # Errors
    ///
    /// This method fails if the field data is not in JSON format
    /// or it cannot be properly deserialized to target type `T`. For more
    /// details please see [`serde_json::from_slice`].
    #[cfg(feature = "json")]
    #[cfg_attr(nightly, doc(cfg(feature = "json")))]
    pub async fn json<T: DeserializeOwned>(self) -> crate::Result<T> {
        serde_json::from_slice(&self.bytes().await?).map_err(crate::Error::DecodeJson)
    }

    /// Get the full field data as text.
    ///
    /// This method decodes the field data with `BOM sniffing` and with
    /// malformed sequences replaced with the `REPLACEMENT CHARACTER`.
    /// Encoding is determined from the `charset` parameter of `Content-Type`
    /// header, and defaults to `utf-8` if not presented.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use bytes::Bytes;
    /// use futures_util::stream::once;
    /// use multer::Multipart;
    ///
    /// # async fn run() {
    /// let data =
    ///     "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
    /// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    /// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    ///
    /// while let Some(field) = multipart.next_field().await.unwrap() {
    ///     let content = field.text().await.unwrap();
    ///     assert_eq!(content, "abcd");
    /// }
    /// # }
    /// # tokio::runtime::Runtime::new().unwrap().block_on(run());
    /// ```
    pub async fn text(self) -> crate::Result<String> {
        self.text_with_charset("utf-8").await
    }

    /// Get the full field data as text given a specific encoding.
    ///
    /// This method decodes the field data with `BOM sniffing` and with
    /// malformed sequences replaced with the `REPLACEMENT CHARACTER`.
    /// You can provide a default encoding for decoding the raw message, while
    /// the `charset` parameter of `Content-Type` header is still prioritized.
    /// For more information about the possible encoding name, please go to
    /// [encoding_rs] docs.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use bytes::Bytes;
    /// use futures_util::stream::once;
    /// use multer::Multipart;
    ///
    /// # async fn run() {
    /// let data =
    ///     "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
    /// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    /// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    ///
    /// while let Some(field) = multipart.next_field().await.unwrap() {
    ///     let content = field.text_with_charset("utf-8").await.unwrap();
    ///     assert_eq!(content, "abcd");
    /// }
    /// # }
    /// # tokio::runtime::Runtime::new().unwrap().block_on(run());
    /// ```
    pub async fn text_with_charset(self, default_encoding: &str) -> crate::Result<String> {
        let encoding_name = self
            .content_type()
            .and_then(|mime| mime.get_param(mime::CHARSET))
            .map(|charset| charset.as_str())
            .unwrap_or(default_encoding);

        let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(UTF_8);

        let bytes = self.bytes().await?;

        let (text, ..) = encoding.decode(&bytes);

        match text {
            Cow::Owned(s) => Ok(s),
            Cow::Borrowed(s) => Ok(String::from(s)),
        }
    }

    /// Get the index of this field in order they appeared in the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use bytes::Bytes;
    /// use futures_util::stream::once;
    /// use multer::Multipart;
    ///
    /// # async fn run() {
    /// let data =
    ///     "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
    /// let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    /// let mut multipart = Multipart::new(stream, "X-BOUNDARY");
    ///
    /// while let Some(field) = multipart.next_field().await.unwrap() {
    ///     let idx = field.index();
    ///     println!("Field index: {}", idx);
    /// }
    /// # }
    /// # tokio::runtime::Runtime::new().unwrap().block_on(run());
    /// ```
    pub fn index(&self) -> usize {
        self.idx
    }
}

impl Stream for Field<'_> {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        debug_assert!(self.state.try_lock().is_some(), "expected exlusive lock");
        let state = self.state.clone();
        let mut lock = match state.try_lock() {
            Some(lock) => lock,
            None => return Poll::Ready(Some(Err(Error::LockFailure))),
        };

        let state = &mut *lock;
        if let Err(err) = state.buffer.poll_stream(cx) {
            return Poll::Ready(Some(Err(crate::Error::StreamReadFailed(err.into()))));
        }

        match state
            .buffer
            .read_field_data(&state.boundary, state.curr_field_name.as_deref())
        {
            Ok(Some((done, bytes))) => {
                state.curr_field_size_counter += bytes.len() as u64;

                if state.curr_field_size_counter > state.curr_field_size_limit {
                    return Poll::Ready(Some(Err(crate::Error::FieldSizeExceeded {
                        limit: state.curr_field_size_limit,
                        field_name: state.curr_field_name.clone(),
                    })));
                }

                if done {
                    state.stage = StreamingStage::ReadingBoundary;
                    self.done = true;
                }

                Poll::Ready(Some(Ok(bytes)))
            }
            Ok(None) => Poll::Pending,
            Err(err) => Poll::Ready(Some(Err(err))),
        }
    }
}
