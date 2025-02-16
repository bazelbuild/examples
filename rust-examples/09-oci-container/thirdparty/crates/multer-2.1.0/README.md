[![Github Actions Status](https://github.com/rousan/multer-rs/workflows/Test/badge.svg)](https://github.com/rousan/multer-rs/actions)
[![crates.io](https://img.shields.io/crates/v/multer.svg)](https://crates.io/crates/multer)
[![Documentation](https://docs.rs/multer/badge.svg)](https://docs.rs/multer)
[![MIT](https://img.shields.io/crates/l/multer.svg)](./LICENSE)

# multer-rs

An async parser for `multipart/form-data` content-type in Rust.

It accepts a [`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html) of [`Bytes`](https://docs.rs/bytes/1/bytes/struct.Bytes.html) as
a source, so that It can be plugged into any async Rust environment e.g. any async server.

[Docs](https://docs.rs/multer)

## Install    

Add this to your `Cargo.toml`:

```toml
[dependencies]
multer = "2.0"
```

# Basic Example

```rust
use bytes::Bytes;
use futures::stream::Stream;
// Import multer types.
use multer::Multipart;
use std::convert::Infallible;
use futures::stream::once;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a byte stream and the boundary from somewhere e.g. server request body.
    let (stream, boundary) = get_byte_stream_from_somewhere().await;

    // Create a `Multipart` instance from that byte stream and the boundary.
    let mut multipart = Multipart::new(stream, boundary);

    // Iterate over the fields, use `next_field()` to get the next field.
    while let Some(mut field) = multipart.next_field().await? {
        // Get field name.
        let name = field.name();
        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();

        println!("Name: {:?}, File Name: {:?}", name, file_name);

        // Process the field data chunks e.g. store them in a file.
        while let Some(chunk) = field.chunk().await? {
            // Do something with field chunk.
            println!("Chunk: {:?}", chunk);
        }
    }

    Ok(())
}

// Generate a byte stream and the boundary from somewhere e.g. server request body.
async fn get_byte_stream_from_somewhere() -> (impl Stream<Item = Result<Bytes, Infallible>>, &'static str) {
    let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY--\r\n";
    let stream = once(async move { Result::<Bytes, Infallible>::Ok(Bytes::from(data)) });
    
    (stream, "X-BOUNDARY")
}
``` 

## Prevent Denial of Service (DoS) Attacks

This crate also provides some APIs to prevent potential DoS attacks with fine grained control. It's recommended to add some constraints
on field (specially text field) size to prevent DoS attacks exhausting the server's memory.

An example:

```rust
use multer::{Multipart, Constraints, SizeLimit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create some constraints to be applied to the fields to prevent DoS attack.
    let constraints = Constraints::new()
         // We only accept `my_text_field` and `my_file_field` fields,
         // For any unknown field, we will throw an error.
         .allowed_fields(vec!["my_text_field", "my_file_field"])
         .size_limit(
             SizeLimit::new()
                 // Set 15mb as size limit for the whole stream body.
                 .whole_stream(15 * 1024 * 1024)
                 // Set 10mb as size limit for all fields.
                 .per_field(10 * 1024 * 1024)
                 // Set 30kb as size limit for our text field only.
                 .for_field("my_text_field", 30 * 1024),
         );

    // Create a `Multipart` instance from a stream and the constraints.
    let mut multipart = Multipart::with_constraints(some_stream, "X-BOUNDARY", constraints);

    while let Some(field) = multipart.next_field().await.unwrap() {
        let content = field.text().await.unwrap();
        assert_eq!(content, "abcd");
    } 
   
    Ok(())
}
```

## Usage with [hyper.rs](https://hyper.rs/) server

An [example](https://github.com/rousan/multer-rs/blob/master/examples/hyper_server_example.rs) showing usage with [hyper.rs](https://hyper.rs/).

For more examples, please visit [examples](https://github.com/rousan/multer-rs/tree/master/examples).

## Contributing

Your PRs and suggestions are always welcome.
