#include "../mem_chr.h"

inline static const uint8_t *_start_chr_64(const uint8_t *s, size_t n, uint8_t c)
{
    const size_t len = n;
    const uint8_t * buf_ptr = s;
    const uint8_t * end_ptr = buf_ptr + len;
    const uint64_t cc64 = (uint64_t)c * 0x0101010101010101LL;
    //
    {
        const size_t unroll = 8;
        const size_t loop_size = 8;
        const uint8_t * end_ptr_8_8 = end_ptr - (loop_size * unroll);
        while (buf_ptr <= end_ptr_8_8) {
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 0);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 1);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 2);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 3);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 4);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 5);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 6);
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 7);
            //
            buf_ptr = buf_ptr + (loop_size * unroll);
        }
    }
    {
        const size_t loop_size = 8;
        const uint8_t * end_ptr_8 = end_ptr - loop_size;
        while (buf_ptr <= end_ptr_8) {
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    return _memchr_remaining_7_bytes_impl(buf_ptr, (uint32_t)cc64, end_ptr);
}

inline static const uint8_t *_start_chr_32(const uint8_t *s, size_t n, uint8_t c)
{
    const size_t len = n;
    const uint8_t * buf_ptr = s;
    const uint8_t * end_ptr = buf_ptr + len;
    const uint32_t cc32 = (uint32_t)c * 0x01010101L;
    //
    {
        const size_t unroll = 8;
        const size_t loop_size = 4;
        const uint8_t * end_ptr_4_8 = end_ptr - (loop_size * unroll);
        while (buf_ptr <= end_ptr_4_8) {
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 0);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 1);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 2);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 3);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 4);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 5);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 6);
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 7);
            //
            buf_ptr = buf_ptr + (loop_size * unroll);
        }
    }
    {
        const size_t loop_size = 4;
        const uint8_t * end_ptr_4 = end_ptr - loop_size;
        while (buf_ptr <= end_ptr_4) {
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 0);
            buf_ptr = buf_ptr + loop_size;
        }
    }
    return _memchr_remaining_3_bytes_impl(buf_ptr, (uint16_t)cc32, end_ptr);
}

const uint8_t *_memchr_basic_impl(const uint8_t *s, size_t n, uint8_t c)
{
#if __SIZEOF_POINTER__ == 8
    return _start_chr_64(s, n, c);
#elif __SIZEOF_POINTER__ == 4
    return _start_chr_32(s, n, c);
#else
#error Undefined __SIZEOF_POINTER__
#endif
}

/*
 * Reference.
 * https://pzemtsov.github.io/2019/09/26/making-a-char-searcher-in-c.html
 * https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
*/

/*
 * The simple implement:

const uint8_t *_memchr_impl(const uint8_t *s, uint8_t c, size_t n)
{
    for (;n>0; n--, s++) {
        if (*s == c) {
            return s;
        }
    }
    return (const uint8_t *)0;
}
*/
