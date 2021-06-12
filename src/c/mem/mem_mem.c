#include <string.h>

extern int _memeq_basic_impl(const unsigned char *a, const unsigned char *b, size_t n);

const unsigned char *_memmem_basic_impl(
    const unsigned char *haystack, size_t haystack_len,
    const unsigned char *needle, size_t needle_len)
{
    if (haystack_len < needle_len) {
        return (const unsigned char *)0;
    }
    for (size_t i=0; i<(haystack_len - needle_len); i++) {
        if (0 == _memeq_basic_impl(&haystack[i], needle, needle_len)) {
            return &haystack[i];
        }
    }
    return (const unsigned char *)0;
}

/*
 * Reference:
 * The naive optimize algorithm for utf8 sequence.
 *   https://github.com/aki-akaguma/naive_opt
*/

/*
 * The simple implement:

#[inline(always)]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    for i in 0..(hay_len - nee_len) {
        if &haystack[i..(i + nee_len)] == needle {
            return Some(i);
        }
    }
    None
}
*/
