#include <string.h>

void _memcpy_basic_impl(unsigned char *dst, const unsigned char *src, size_t n)
{
    for (; n>0; n--, dst++, src++) {
        *dst = *src;
    }
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if dst.len() < src.len() {
        return Err(RangeError);
    }
    for i in 0..src.len() {
        dst[i] = src[i];
    }
    Ok(())
}

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    let end_ptr = unsafe { dst_ptr.add(src_len) };
    while dst_ptr < end_ptr {
        unsafe {
            *dst_ptr = *src_ptr;
        }
        dst_ptr = unsafe { dst_ptr.add(1) };
        src_ptr = unsafe { src_ptr.add(1) };
    }
    Ok(())
}
*/
