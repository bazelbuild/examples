# Examples of using multer-rs

These examples show of how to do common tasks using `multer-rs`.

Please visit: [Docs](https://docs.rs/multer) for the documentation.

Run an example:

```sh
 cargo run --example example_name
```

* [`simple_example`](simple_example.rs) - A basic example using `multer`.

* [`hyper_server_example`](hyper_server_example.rs) - Shows how to use this crate with Rust HTTP server [hyper](https://hyper.rs/).

* [`parse_async_read`](parse_async_read.rs) - Shows how to parse `multipart/form-data` from an [`AsyncRead`](https://docs.rs/tokio/1/tokio/io/trait.AsyncRead.html).

* [`prevent_dos_attack`](prevent_dos_attack.rs) - Shows how to apply some rules to prevent potential DoS attacks while handling `multipart/form-data`.
