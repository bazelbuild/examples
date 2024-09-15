use http::header::{self, HeaderMap};

use crate::constants::ContentDispositionAttr;

#[derive(Debug)]
pub(crate) struct ContentDisposition {
    pub(crate) field_name: Option<String>,
    pub(crate) file_name: Option<String>,
}

impl ContentDisposition {
    pub fn parse(headers: &HeaderMap) -> ContentDisposition {
        let content_disposition = headers.get(header::CONTENT_DISPOSITION).map(|val| val.as_bytes());

        let field_name = content_disposition
            .and_then(|val| ContentDispositionAttr::Name.extract_from(val))
            .and_then(|attr| std::str::from_utf8(attr).ok())
            .map(String::from);

        let file_name = content_disposition
            .and_then(|val| ContentDispositionAttr::FileName.extract_from(val))
            .and_then(|attr| std::str::from_utf8(attr).ok())
            .map(String::from);

        ContentDisposition { field_name, file_name }
    }
}
