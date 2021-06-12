#include <string.h>

extern int _memeq_basic_impl(const unsigned char *a, const unsigned char *b, size_t n);

const unsigned char *_memrmem_basic_impl(
    const unsigned char *haystack, size_t haystack_len,
    const unsigned char *needle, size_t needle_len)
{
    if (haystack_len < needle_len) {
        return (const unsigned char *)0;
    }
    for (size_t i=(haystack_len - needle_len -1); i>0; i--) {
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
pub fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    let max_i = hay_len - nee_len;
    for i in 0..max_i {
        let j = max_i - i;
        if &haystack[j..(j + nee_len)] == needle {
            return Some(j);
        }
    }
    None
}
*/
