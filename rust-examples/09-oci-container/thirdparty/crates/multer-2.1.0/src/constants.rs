pub(crate) const DEFAULT_WHOLE_STREAM_SIZE_LIMIT: u64 = std::u64::MAX;
pub(crate) const DEFAULT_PER_FIELD_SIZE_LIMIT: u64 = std::u64::MAX;

pub(crate) const MAX_HEADERS: usize = 32;
pub(crate) const BOUNDARY_EXT: &str = "--";
pub(crate) const CR: &str = "\r";
#[allow(dead_code)]
pub(crate) const LF: &str = "\n";
pub(crate) const CRLF: &str = "\r\n";
pub(crate) const CRLF_CRLF: &str = "\r\n\r\n";

#[derive(PartialEq)]
pub(crate) enum ContentDispositionAttr {
    Name,
    FileName,
}

impl ContentDispositionAttr {
    /// Extract ContentDisposition Attribute from header.
    ///
    /// Some older clients may not quote the name or filename, so we allow them
    pub fn extract_from<'h>(&self, header: &'h [u8]) -> Option<&'h [u8]> {
        let prefix = match self {
            ContentDispositionAttr::Name => &b"name="[..],
            ContentDispositionAttr::FileName => &b"filename="[..],
        };

        if let Some(i) = memchr::memmem::find(header, prefix) {
            // Check if this is malformed, with `filename` coming first.
            if *self == ContentDispositionAttr::Name && i > 0 && header[i - 1] == b'e' {
                return None;
            }

            // Handle quoted strings first, then unquoted string.
            // FIXME: According to RFC6266 4.1, a 'quoted-string' (RFC 2616 2.2)
            // can contain a 'quoted-pair', which can be used to escape a quote
            // character in a name with `\`. That is, "a\"b" is a valid name.
            // But this routine would truncate it to `a\`; this is wrong.
            let rest = &header[(i + prefix.len())..];
            if rest.starts_with(b"\"") {
                let k = memchr::memchr(b'"', &rest[1..])?;
                return Some(&rest[1..(k + 1)]);
            } else {
                let j = memchr::memchr(b';', rest).unwrap_or(rest.len());
                return Some(&rest[..j]);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_disposition_name_only() {
        let val = br#"form-data; name="my_field""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"my_field");
        assert!(filename.is_none());
    }

    #[test]
    fn test_content_disposition_extraction() {
        let val = br#"form-data; name="my_field"; filename="file abc.txt""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"my_field");
        assert_eq!(filename.unwrap(), b"file abc.txt");

        let val = "form-data; name=\"你好\"; filename=\"file abc.txt\"".as_bytes();
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), "你好".as_bytes());
        assert_eq!(filename.unwrap(), b"file abc.txt");

        let val = "form-data; name=\"কখগ\"; filename=\"你好.txt\"".as_bytes();
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), "কখগ".as_bytes());
        assert_eq!(filename.unwrap(), "你好.txt".as_bytes());
    }

    #[test]
    fn test_content_disposition_file_name_only() {
        // These are technically malformed, as RFC 7578 says the `name`
        // parameter _must_ be included. But okay.
        let val = br#"form-data; filename="file-name.txt""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(filename.unwrap(), b"file-name.txt");
        assert!(name.is_none());

        let val = "form-data; filename=\"কখগ-你好.txt\"".as_bytes();
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(filename.unwrap(), "কখগ-你好.txt".as_bytes());
        assert!(name.is_none());
    }

    #[test]
    fn test_content_disposition_name_unquoted() {
        let val = br#"form-data; name=my_field"#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"my_field");
        assert!(filename.is_none());

        let val = br#"form-data; name=my_field; filename=file-name.txt"#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"my_field");
        assert_eq!(filename.unwrap(), b"file-name.txt");
    }

    #[test]
    fn test_content_disposition_name_quoted() {
        let val = br#"form-data; name="my;f;ield""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"my;f;ield");
        assert!(filename.is_none());

        let val = br#"form-data; name=my_field; filename="file;name.txt""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"my_field");
        assert_eq!(filename.unwrap(), b"file;name.txt");

        let val = br#"form-data; name=; filename=filename.txt"#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b"");
        assert_eq!(filename.unwrap(), b"filename.txt");

        let val = br#"form-data; name=";"; filename=";""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        let filename = ContentDispositionAttr::FileName.extract_from(val);
        assert_eq!(name.unwrap(), b";");
        assert_eq!(filename.unwrap(), b";");
    }

    // FIXME: This test should pass.
    #[test]
    #[should_panic]
    fn test_content_disposition_name_escaped_quote() {
        let val = br#"form-data; name="my\"field\"name""#;
        let name = ContentDispositionAttr::Name.extract_from(val);
        assert_eq!(name.unwrap(), b"my\"field\"name");
    }
}
