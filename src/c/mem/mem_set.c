#include <string.h>

void _memset_basic_impl(unsigned char *s, size_t n, unsigned char c)
{
    for (; n>0; n--, s++) {
        *s = c;
    }
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    if buf.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        buf[i] = c;
    }
    Ok(())
}
*/
