#include <string.h>

int _memeq_basic_impl(const unsigned char *a, const unsigned char *b, size_t n)
{
    for (; n>0; n--, a++, b++) {
        if (*a != *b) {
            return !0;
        }
    }
    return 0;
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    for i in 0..a_len {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let min_len = a_len;
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    while a_ptr < end_ptr {
        let aa = unsafe { *a_ptr };
        let bb = unsafe { *b_ptr };
        if aa != bb {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(1) };
        b_ptr = unsafe { b_ptr.add(1) };
    }
    true
}
*/
