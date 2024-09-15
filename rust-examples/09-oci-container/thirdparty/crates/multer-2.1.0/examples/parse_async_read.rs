use multer::Multipart;
use tokio::io::AsyncRead;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate an `AsyncRead` and the boundary from somewhere e.g. server request
    // body.
    let (reader, boundary) = get_async_reader_from_somewhere().await;

    // Create a `Multipart` instance from that async reader and the boundary.
    let mut multipart = Multipart::with_reader(reader, boundary);

    // Iterate over the fields, use `next_field()` to get the next field.
    while let Some(mut field) = multipart.next_field().await? {
        // Get field name.
        let name = field.name();
        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();

        println!("Name: {:?}, File Name: {:?}", name, file_name);

        // Process the field data chunks e.g. store them in a file.
        let mut field_bytes_len = 0;
        while let Some(field_chunk) = field.chunk().await? {
            // Do something with field chunk.
            field_bytes_len += field_chunk.len();
        }

        println!("Field Bytes Length: {:?}", field_bytes_len);
    }

    Ok(())
}

// Generate an `AsyncRead` and the boundary from somewhere e.g. server request
// body.
async fn get_async_reader_from_somewhere() -> (impl AsyncRead, &'static str) {
    let data = "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_text_field\"\r\n\r\nabcd\r\n--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"my_file_field\"; filename=\"a-text-file.txt\"\r\nContent-Type: text/plain\r\n\r\nHello world\nHello\r\nWorld\rAgain\r\n--X-BOUNDARY--\r\n";

    (data.as_bytes(), "X-BOUNDARY")
}
