use core::convert::TryInto;
use core::convert::TryFrom;

#[allow(missing_docs)]
pub struct Bytes<'a> {
    start: *const u8,
    end: *const u8,
    /// INVARIANT: start <= cursor && cursor <= end
    cursor: *const u8,
    phantom: core::marker::PhantomData<&'a ()>,
}

#[allow(missing_docs)]
impl<'a> Bytes<'a> {
    #[inline]
    pub fn new(slice: &'a [u8]) -> Bytes<'a> {
        let start = slice.as_ptr();
        // SAFETY: obtain pointer to slice end; start points to slice start.
        let end = unsafe { start.add(slice.len()) };
        let cursor = start;
        Bytes {
            start,
            end,
            cursor,
            phantom: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.cursor as usize - self.start as usize
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.cursor < self.end {
            // SAFETY:  bounds checked
            Some(unsafe { *self.cursor })
        } else {
            None
        }
    }

    #[inline]
    pub fn peek_ahead(&self, n: usize) -> Option<u8> {
        // SAFETY: obtain a potentially OOB pointer that is later compared against the `self.end`
        // pointer.
        let ptr = unsafe { self.cursor.add(n) };
        if ptr < self.end {
            // SAFETY: bounds checked pointer dereference is safe
            Some(unsafe { *ptr })
        } else {
            None
        }
    }
    
    #[inline]
    pub fn peek_n<'b: 'a, U: TryFrom<&'a [u8]>>(&'b self, n: usize) -> Option<U> {
        // TODO: once we bump MSRC, use const generics to allow only [u8; N] reads
        // TODO: drop `n` arg in favour of const
        // let n = core::mem::size_of::<U>();
        self.as_ref().get(..n)?.try_into().ok()
    }

    /// Advance by 1, equivalent to calling `advance(1)`.
    ///
    /// # Safety
    /// 
    /// Caller must ensure that Bytes hasn't been advanced/bumped by more than [`Bytes::len()`].
    #[inline]
    pub unsafe fn bump(&mut self) {
        self.advance(1)
    }

    /// Advance cursor by `n`
    ///
    /// # Safety
    /// 
    /// Caller must ensure that Bytes hasn't been advanced/bumped by more than [`Bytes::len()`].
    #[inline]
    pub unsafe fn advance(&mut self, n: usize) {
        self.cursor = self.cursor.add(n);
        debug_assert!(self.cursor <= self.end, "overflow");
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end as usize - self.cursor as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn slice(&mut self) -> &'a [u8] {
        // SAFETY: not moving position at all, so it's safe
        let slice = unsafe { slice_from_ptr_range(self.start, self.cursor) };
        self.commit();
        slice
    }

    // TODO: this is an anti-pattern, should be removed
    /// Deprecated. Do not use!
    /// # Safety
    /// 
    /// Caller must ensure that `skip` is at most the number of advances (i.e., `bytes.advance(3)`
    /// implies a skip of at most 3).
    #[inline]
    pub unsafe fn slice_skip(&mut self, skip: usize) -> &'a [u8] {
        debug_assert!(self.cursor.sub(skip) >= self.start);
        let head = slice_from_ptr_range(self.start, self.cursor.sub(skip));
        self.commit();
        head
    }
    
    #[inline]
    pub fn commit(&mut self) {
        self.start = self.cursor
    }

    /// # Safety
    /// 
    /// see [`Bytes::advance`] safety comment.
    #[inline]
    pub unsafe fn advance_and_commit(&mut self, n: usize) {
        self.advance(n);
        self.commit();
    }
    
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.cursor
    }

    #[inline]
    pub fn start(&self) -> *const u8 {
        self.start
    }
    
    #[inline]
    pub fn end(&self) -> *const u8 {
        self.end
    }
    
    /// # Safety
    /// 
    /// Must ensure invariant `bytes.start() <= ptr && ptr <= bytes.end()`.
    #[inline]
    pub unsafe fn set_cursor(&mut self, ptr: *const u8) {
        debug_assert!(ptr >= self.start);
        debug_assert!(ptr <= self.end);
        self.cursor = ptr;
    }
}

impl<'a> AsRef<[u8]> for Bytes<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        // SAFETY: not moving position at all, so it's safe
        unsafe { slice_from_ptr_range(self.cursor, self.end) }
    }
}

/// # Safety
///
/// Must ensure start and end point to the same memory object to uphold memory safety.
#[inline]
unsafe fn slice_from_ptr_range<'a>(start: *const u8, end: *const u8) -> &'a [u8] {
    debug_assert!(start <= end);
    core::slice::from_raw_parts(start, end as usize - start as usize)
}

impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        if self.cursor < self.end {
            // SAFETY: bounds checked dereference
            unsafe {
                let b = *self.cursor;
                self.bump();
                Some(b)
            }
        } else {
            None
        }
    }
}
