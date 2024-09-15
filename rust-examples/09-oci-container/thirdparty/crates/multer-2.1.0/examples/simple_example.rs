use std::convert::Infallible;

use bytes::Bytes;
use futures_util::stream::Stream;
// Import multer types.
use multer::Multipart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a byte stream and the boundary from somewhere e.g. server request
    // body.
    let (stream, boundary) = get_byte_stream_from_somewhere().await;

    // Create a `Multipart` instance from that byte stream and the boundary.
    let mut multipart = Multipart::new(stream, boundary);

    // Iterate over the fields, use `next_field()` to get the next field.
    while let Some(field) = multipart.next_field().await? {
        // Get field name.
        let name = field.name();
        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();

        println!("Name: {:?}, File Name: {:?}", name, file_name);

        // Read field content as text.
        let content = field.text().await?;
        println!("Content: {:?}", content);
    }

    Ok(())
}

// Generate a byte stream and the boundary from somewhere e.g. server request
// body.
async fn get_byte_stream_from_somewhere() -> (impl Stream<Item = Result<Bytes, Infallible>>, &'static str) {
    let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_file_field\"; filename=\"a-text-file.txt\"\r\nContent-Type: text/plain\r\n\r\nHello world\nHello\r\nWorld\rAgain\r\n--X-BOUNDARY--\r\n";
    let stream = futures_util::stream::iter(
        data.chars()
            .map(|ch| ch.to_string())
            .map(|part| Ok(Bytes::copy_from_slice(part.as_bytes()))),
    );

    (stream, "X-BOUNDARY")
}
