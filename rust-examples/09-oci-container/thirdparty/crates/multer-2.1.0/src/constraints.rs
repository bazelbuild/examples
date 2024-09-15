use crate::size_limit::SizeLimit;

/// Represents some rules to be applied on the stream and field's content size
/// to prevent DoS attacks.
///
/// It's recommended to add some rules on field (specially text field) size to
/// avoid potential DoS attacks from attackers running the server out of memory.
/// This type provides some API to apply constraints on very granular level to
/// make `multipart/form-data` safe. By default, it does not apply any
/// constraint.
///
/// # Examples
///
/// ```
/// use multer::{Multipart, Constraints, SizeLimit};
/// # use bytes::Bytes;
/// # use std::convert::Infallible;
/// # use futures_util::stream::once;
///
/// # async fn run() {
/// # let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
/// # let some_stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
/// // Create some constraints to be applied to the fields to prevent DoS attack.
/// let constraints = Constraints::new()
///      // We only accept `my_text_field` and `my_file_field` fields,
///      // For any unknown field, we will throw an error.
///      .allowed_fields(vec!["my_text_field", "my_file_field"])
///      .size_limit(
///          SizeLimit::new()
///              // Set 15mb as size limit for the whole stream body.
///              .whole_stream(15 * 1024 * 1024)
///              // Set 10mb as size limit for all fields.
///              .per_field(10 * 1024 * 1024)
///              // Set 30kb as size limit for our text field only.
///              .for_field("my_text_field", 30 * 1024),
///      );
///
/// // Create a `Multipart` instance from a stream and the constraints.
/// let mut multipart = Multipart::with_constraints(some_stream, "X-BOUNDARY", constraints);
///
/// while let Some(field) = multipart.next_field().await.unwrap() {
///     let content = field.text().await.unwrap();
///     assert_eq!(content, "abcd");
/// }
/// # }
/// # tokio::runtime::Runtime::new().unwrap().block_on(run());
/// ```
#[derive(Debug, Default)]
pub struct Constraints {
    pub(crate) size_limit: SizeLimit,
    pub(crate) allowed_fields: Option<Vec<String>>,
}

impl Constraints {
    /// Creates a set of rules with default behaviour.
    pub fn new() -> Constraints {
        Constraints::default()
    }

    /// Applies rules on field's content length.
    pub fn size_limit(self, size_limit: SizeLimit) -> Constraints {
        Constraints {
            size_limit,
            allowed_fields: self.allowed_fields,
        }
    }

    /// Specify which fields should be allowed, for any unknown field, the
    /// [`next_field`](crate::Multipart::next_field) will throw an error.
    pub fn allowed_fields<N: Into<String>>(self, allowed_fields: Vec<N>) -> Constraints {
        let allowed_fields = allowed_fields.into_iter().map(|item| item.into()).collect();

        Constraints {
            size_limit: self.size_limit,
            allowed_fields: Some(allowed_fields),
        }
    }

    pub(crate) fn is_it_allowed(&self, field: Option<&str>) -> bool {
        if let Some(ref allowed_fields) = self.allowed_fields {
            field
                .map(|field| allowed_fields.iter().any(|item| item == field))
                .unwrap_or(false)
        } else {
            true
        }
    }
}
