use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, Bytes, BytesMut};
use futures_util::stream::Stream;

use crate::constants;

pub(crate) struct StreamBuffer<'r> {
    pub(crate) eof: bool,
    pub(crate) buf: BytesMut,
    pub(crate) stream: Pin<Box<dyn Stream<Item = Result<Bytes, crate::Error>> + Send + 'r>>,
    pub(crate) whole_stream_size_limit: u64,
    pub(crate) stream_size_counter: u64,
}

impl<'r> StreamBuffer<'r> {
    pub fn new<S>(stream: S, whole_stream_size_limit: u64) -> Self
    where
        S: Stream<Item = Result<Bytes, crate::Error>> + Send + 'r,
    {
        StreamBuffer {
            eof: false,
            buf: BytesMut::new(),
            stream: Box::pin(stream),
            whole_stream_size_limit,
            stream_size_counter: 0,
        }
    }

    pub fn poll_stream(&mut self, cx: &mut Context<'_>) -> Result<(), crate::Error> {
        if self.eof {
            return Ok(());
        }

        loop {
            match self.stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(data))) => {
                    self.stream_size_counter += data.len() as u64;

                    if self.stream_size_counter > self.whole_stream_size_limit {
                        return Err(crate::Error::StreamSizeExceeded {
                            limit: self.whole_stream_size_limit,
                        });
                    }

                    self.buf.extend_from_slice(&data)
                }
                Poll::Ready(Some(Err(err))) => return Err(err),
                Poll::Ready(None) => {
                    self.eof = true;
                    return Ok(());
                }
                Poll::Pending => return Ok(()),
            }
        }
    }

    pub fn read_exact(&mut self, size: usize) -> Option<Bytes> {
        if size <= self.buf.len() {
            Some(self.buf.split_to(size).freeze())
        } else {
            None
        }
    }

    pub fn peek_exact(&mut self, size: usize) -> Option<&[u8]> {
        self.buf.get(..size)
    }

    pub fn read_until(&mut self, pattern: &[u8]) -> Option<Bytes> {
        memchr::memmem::find(&self.buf, pattern).map(|idx| self.buf.split_to(idx + pattern.len()).freeze())
    }

    pub fn read_to(&mut self, pattern: &[u8]) -> Option<Bytes> {
        memchr::memmem::find(&self.buf, pattern).map(|idx| self.buf.split_to(idx).freeze())
    }

    pub fn advance_past_transport_padding(&mut self) -> bool {
        match self.buf.iter().position(|b| *b != b' ' && *b != b'\t') {
            Some(pos) => {
                self.buf.advance(pos);
                true
            }
            None => {
                self.buf.clear();
                false
            }
        }
    }

    pub fn read_field_data(
        &mut self,
        boundary: &str,
        field_name: Option<&str>,
    ) -> crate::Result<Option<(bool, Bytes)>> {
        log::trace!("finding next field: {:?}", field_name);
        if self.buf.is_empty() && self.eof {
            log::trace!("empty buffer && EOF");
            return Err(crate::Error::IncompleteFieldData {
                field_name: field_name.map(|s| s.to_owned()),
            });
        } else if self.buf.is_empty() {
            return Ok(None);
        }

        let boundary_deriv = format!("{}{}{}", constants::CRLF, constants::BOUNDARY_EXT, boundary);
        let b_len = boundary_deriv.len();

        match memchr::memmem::find(&self.buf, boundary_deriv.as_bytes()) {
            Some(idx) => {
                log::trace!("new field found at {}", idx);
                let bytes = self.buf.split_to(idx).freeze();

                // discard \r\n.
                self.buf.advance(constants::CRLF.len());

                Ok(Some((true, bytes)))
            }
            None if self.eof => {
                log::trace!("no new field found: EOF. terminating");
                Err(crate::Error::IncompleteFieldData {
                    field_name: field_name.map(|s| s.to_owned()),
                })
            }
            None => {
                let buf_len = self.buf.len();
                let rem_boundary_part_max_len = b_len - 1;
                let rem_boundary_part_idx = if buf_len >= rem_boundary_part_max_len {
                    buf_len - rem_boundary_part_max_len
                } else {
                    0
                };

                log::trace!("no new field found, not EOF, checking close");
                let bytes = &self.buf[rem_boundary_part_idx..];
                match memchr::memmem::rfind(bytes, constants::CR.as_bytes()) {
                    Some(rel_idx) => {
                        let idx = rel_idx + rem_boundary_part_idx;

                        match memchr::memmem::find(boundary_deriv.as_bytes(), &self.buf[idx..]) {
                            Some(_) => {
                                let bytes = self.buf.split_to(idx).freeze();

                                match bytes.is_empty() {
                                    true => Ok(None),
                                    false => Ok(Some((false, bytes))),
                                }
                            }
                            None => Ok(Some((false, self.read_full_buf()))),
                        }
                    }
                    None => Ok(Some((false, self.read_full_buf()))),
                }
            }
        }
    }

    pub fn read_full_buf(&mut self) -> Bytes {
        self.buf.split_to(self.buf.len()).freeze()
    }
}

impl fmt::Debug for StreamBuffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamBuffer").finish()
    }
}
