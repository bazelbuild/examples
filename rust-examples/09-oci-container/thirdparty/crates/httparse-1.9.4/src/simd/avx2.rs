use crate::iter::Bytes;

#[inline]
#[target_feature(enable = "avx2", enable = "sse4.2")]
pub unsafe fn match_uri_vectored(bytes: &mut Bytes) {
    while bytes.as_ref().len() >= 32 {
        let advance = match_url_char_32_avx(bytes.as_ref());
        bytes.advance(advance);

        if advance != 32 {
            return;
        }
    }
    // do both, since avx2 only works when bytes.len() >= 32
    super::sse42::match_uri_vectored(bytes)
}

#[inline(always)]
#[allow(non_snake_case, overflowing_literals)]
#[allow(unused)]
unsafe fn match_url_char_32_avx(buf: &[u8]) -> usize {
    debug_assert!(buf.len() >= 32);

    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    let ptr = buf.as_ptr();

    let LSH: __m256i = _mm256_set1_epi8(0x0f);

    // See comment in sse42::match_url_char_16_sse.

    let URI: __m256i = _mm256_setr_epi8(
        0xf8, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc,
        0xfc, 0xfc, 0xfc, 0xfc, 0xf4, 0xfc, 0xf4, 0x7c,
        0xf8, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc,
        0xfc, 0xfc, 0xfc, 0xfc, 0xf4, 0xfc, 0xf4, 0x7c,
    );
    let ARF: __m256i = _mm256_setr_epi8(
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    );

    let data = _mm256_lddqu_si256(ptr as *const _);
    let rbms = _mm256_shuffle_epi8(URI, data);
    let cols = _mm256_and_si256(LSH, _mm256_srli_epi16(data, 4));
    let bits = _mm256_and_si256(_mm256_shuffle_epi8(ARF, cols), rbms);

    let v = _mm256_cmpeq_epi8(bits, _mm256_setzero_si256());
    let r = _mm256_movemask_epi8(v) as u32;

    r.trailing_zeros() as usize
}

#[target_feature(enable = "avx2", enable = "sse4.2")]
pub unsafe fn match_header_value_vectored(bytes: &mut Bytes) {
    while bytes.as_ref().len() >= 32 {
        let advance = match_header_value_char_32_avx(bytes.as_ref());
        bytes.advance(advance);

        if advance != 32 {
            return;
        }
    }
    // do both, since avx2 only works when bytes.len() >= 32
    super::sse42::match_header_value_vectored(bytes)
}

#[inline(always)]
#[allow(non_snake_case)]
#[allow(unused)]
unsafe fn match_header_value_char_32_avx(buf: &[u8]) -> usize {
    debug_assert!(buf.len() >= 32);

    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    let ptr = buf.as_ptr();

    // %x09 %x20-%x7e %x80-%xff
    let TAB: __m256i = _mm256_set1_epi8(0x09);
    let DEL: __m256i = _mm256_set1_epi8(0x7f);
    let LOW: __m256i = _mm256_set1_epi8(0x20);

    let dat = _mm256_lddqu_si256(ptr as *const _);
    // unsigned comparison dat >= LOW
    let low = _mm256_cmpeq_epi8(_mm256_max_epu8(dat, LOW), dat);
    let tab = _mm256_cmpeq_epi8(dat, TAB);
    let del = _mm256_cmpeq_epi8(dat, DEL);
    let bit = _mm256_andnot_si256(del, _mm256_or_si256(low, tab));
    let res = _mm256_movemask_epi8(bit) as u32;
    // TODO: use .trailing_ones() once MSRV >= 1.46
    (!res).trailing_zeros() as usize
}

#[test]
fn avx2_code_matches_uri_chars_table() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }

    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        assert!(byte_is_allowed(b'_', match_uri_vectored));

        for (b, allowed) in crate::URI_MAP.iter().cloned().enumerate() {
            assert_eq!(
                byte_is_allowed(b as u8, match_uri_vectored), allowed,
                "byte_is_allowed({:?}) should be {:?}", b, allowed,
            );
        }
    }
}

#[test]
fn avx2_code_matches_header_value_chars_table() {
    if !is_x86_feature_detected!("avx2") {
        return;
    }

    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        assert!(byte_is_allowed(b'_', match_header_value_vectored));

        for (b, allowed) in crate::HEADER_VALUE_MAP.iter().cloned().enumerate() {
            assert_eq!(
                byte_is_allowed(b as u8, match_header_value_vectored), allowed,
                "byte_is_allowed({:?}) should be {:?}", b, allowed,
            );
        }
    }
}

#[cfg(test)]
unsafe fn byte_is_allowed(byte: u8, f: unsafe fn(bytes: &mut Bytes<'_>)) -> bool {
    let slice = [
        b'_', b'_', b'_', b'_',
        b'_', b'_', b'_', b'_',
        b'_', b'_', b'_', b'_',
        b'_', b'_', b'_', b'_',
        b'_', b'_', b'_', b'_',
        b'_', b'_', b'_', b'_',
        b'_', b'_', byte, b'_',
        b'_', b'_', b'_', b'_',
    ];
    let mut bytes = Bytes::new(&slice);

    f(&mut bytes);

    match bytes.pos() {
        32 => true,
        26 => false,
        _ => unreachable!(),
    }
}
