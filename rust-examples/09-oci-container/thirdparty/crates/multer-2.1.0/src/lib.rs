//! An async parser for `multipart/form-data` content-type in Rust.
//!
//! It accepts a [`Stream`](futures_util::stream::Stream) of
//! [`Bytes`](bytes::Bytes), or with the `tokio-io` feature enabled, an
//! `AsyncRead` reader as a source, so that it can be plugged into any async
//! Rust environment e.g. any async server.
//!
//! # Examples
//!
//! ```no_run
//! use std::convert::Infallible;
//!
//! use bytes::Bytes;
//! // Import multer types.
//! use futures_util::stream::once;
//! use futures_util::stream::Stream;
//! use multer::Multipart;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Generate a byte stream and the boundary from somewhere e.g. server request body.
//!     let (stream, boundary) = get_byte_stream_from_somewhere().await;
//!
//!     // Create a `Multipart` instance from that byte stream and the boundary.
//!     let mut multipart = Multipart::new(stream, boundary);
//!
//!     // Iterate over the fields, use `next_field()` to get the next field.
//!     while let Some(mut field) = multipart.next_field().await? {
//!         // Get field name.
//!         let name = field.name();
//!         // Get the field's filename if provided in "Content-Disposition" header.
//!         let file_name = field.file_name();
//!
//!         println!("Name: {:?}, File Name: {:?}", name, file_name);
//!
//!         // Process the field data chunks e.g. store them in a file.
//!         while let Some(chunk) = field.chunk().await? {
//!             // Do something with field chunk.
//!             println!("Chunk: {:?}", chunk);
//!         }
//!     }
//!
//!     Ok(())
//! }
//!
//! // Generate a byte stream and the boundary from somewhere e.g. server request body.
//! async fn get_byte_stream_from_somewhere(
//! ) -> (impl Stream<Item = Result<Bytes, Infallible>>, &'static str) {
//!     let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; \
//!         name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
//!
//!     let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
//!     (stream, "X-BOUNDARY")
//! }
//! ```
//!
//! ## Prevent Denial of Service (DoS) Attack
//!
//! This crate also provides some APIs to prevent potential DoS attacks with
//! fine grained control. It's recommended to add some constraints
//! on field (specially text field) size to avoid potential DoS attacks from
//! attackers running the server out of memory.
//!
//! An example:
//!
//! ```
//! use multer::{Constraints, Multipart, SizeLimit};
//! # use bytes::Bytes;
//! # use std::convert::Infallible;
//! # use futures_util::stream::once;
//!
//! # async fn run() {
//! # let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; \
//! #   name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
//! # let some_stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
//! // Create some constraints to be applied to the fields to prevent DoS attack.
//! let constraints = Constraints::new()
//!     // We only accept `my_text_field` and `my_file_field` fields,
//!     // For any unknown field, we will throw an error.
//!     .allowed_fields(vec!["my_text_field", "my_file_field"])
//!     .size_limit(
//!         SizeLimit::new()
//!             // Set 15mb as size limit for the whole stream body.
//!             .whole_stream(15 * 1024 * 1024)
//!             // Set 10mb as size limit for all fields.
//!             .per_field(10 * 1024 * 1024)
//!             // Set 30kb as size limit for our text field only.
//!             .for_field("my_text_field", 30 * 1024),
//!     );
//!
//! // Create a `Multipart` instance from a stream and the constraints.
//! let mut multipart = Multipart::with_constraints(some_stream, "X-BOUNDARY", constraints);
//!
//! while let Some(field) = multipart.next_field().await.unwrap() {
//!     let content = field.text().await.unwrap();
//!     assert_eq!(content, "abcd");
//! }
//! # }
//! # tokio::runtime::Runtime::new().unwrap().block_on(run());
//! ```
//!
//! Please refer [`Constraints`] for more info.
//!
//! ## Usage with [hyper.rs](https://hyper.rs/) server
//!
//! An [example](https://github.com/rousan/multer-rs/blob/master/examples/hyper_server_example.rs) showing usage with [hyper.rs](https://hyper.rs/).
//!
//! For more examples, please visit [examples](https://github.com/rousan/multer-rs/tree/master/examples).

#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    trivial_casts,
    unused_qualifications
)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![doc(test(attr(deny(rust_2018_idioms, warnings))))]
#![doc(test(attr(allow(unused_extern_crates, unused_variables))))]

pub use bytes;
pub use constraints::Constraints;
pub use error::Error;
pub use field::Field;
pub use multipart::Multipart;
pub use size_limit::SizeLimit;

mod buffer;
mod constants;
mod constraints;
mod content_disposition;
mod error;
mod field;
mod helpers;
mod multipart;
mod size_limit;

/// A Result type often returned from methods that can have `multer` errors.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Parses the `Content-Type` header to extract the boundary value.
///
/// # Examples
///
/// ```
/// # fn run(){
/// let content_type = "multipart/form-data; boundary=ABCDEFG";
///
/// assert_eq!(
///     multer::parse_boundary(content_type),
///     Ok("ABCDEFG".to_owned())
/// );
/// # }
/// # run();
/// ```
pub fn parse_boundary<T: AsRef<str>>(content_type: T) -> Result<String> {
    let m = content_type
        .as_ref()
        .parse::<mime::Mime>()
        .map_err(crate::Error::DecodeContentType)?;

    if !(m.type_() == mime::MULTIPART && m.subtype() == mime::FORM_DATA) {
        return Err(crate::Error::NoMultipart);
    }

    m.get_param(mime::BOUNDARY)
        .map(|name| name.as_str().to_owned())
        .ok_or(crate::Error::NoBoundary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boundary() {
        let content_type = "multipart/form-data; boundary=ABCDEFG";
        assert_eq!(parse_boundary(content_type), Ok("ABCDEFG".to_owned()));

        let content_type = "multipart/form-data; boundary=------ABCDEFG";
        assert_eq!(parse_boundary(content_type), Ok("------ABCDEFG".to_owned()));

        let content_type = "boundary=------ABCDEFG";
        assert!(parse_boundary(content_type).is_err());

        let content_type = "text/plain";
        assert!(parse_boundary(content_type).is_err());

        let content_type = "text/plain; boundary=------ABCDEFG";
        assert!(parse_boundary(content_type).is_err());
    }
}
