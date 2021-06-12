#include "base.h"

inline static const uint8_t *_chr_c8(const uint8_t *buf_ptr, uint64_t c8)
{
    const uint64_t *aa_ptr = (const uint64_t *)buf_ptr;
    const uint64_t v = (*aa_ptr) ^ c8;
    const uint64_t bits = (v - 0x0101010101010101LL) & ~v & 0x8080808080808080LL;
    if (bits) {
        return buf_ptr + (trailing_zeros_u64(bits) / 8);
    } else {
        return (const uint8_t *)0;
    }
}

inline static const uint8_t *_chr_c4(const uint8_t *buf_ptr, uint32_t c4)
{
    const uint32_t *aa_ptr = (const uint32_t *)buf_ptr;
    const uint32_t v = (*aa_ptr) ^ c4;
    const uint32_t bits = (v - 0x01010101L) & ~v & 0x80808080L;
    if (bits) {
        return buf_ptr + (trailing_zeros_u32(bits) / 8);
    } else {
        return (const uint8_t *)0;
    }
}

inline static const uint8_t *_chr_c2(const uint8_t *buf_ptr, uint16_t c2)
{
    const uint16_t *aa_ptr = (const uint16_t *)buf_ptr;
    const uint16_t v = (*aa_ptr) ^ c2;
    const uint16_t bits = (v - 0x0101) & ~v & 0x8080;
    if (bits) {
        return buf_ptr + (trailing_zeros_u16(bits) / 8);
    } else {
        return (const uint8_t *)0;
    }
}

inline static const uint8_t *_chr_c1(const uint8_t *buf_ptr, uint8_t c1)
{
    if ((*buf_ptr) == c1) {
        return buf_ptr;
    } else {
        return (const uint8_t *)0;
    }
}

#define _unroll_one_chr_8(a_ptr, cc, loop_size, idx) {\
        const uint8_t *aa_ptr = (a_ptr) + ((loop_size) * (idx));\
        const uint8_t *r = _chr_c8(aa_ptr, (cc));\
        if (r) return r;\
    }

#define _unroll_one_chr_4(a_ptr, cc, loop_size, idx) {\
        const uint8_t *aa_ptr = (a_ptr) + ((loop_size) * (idx));\
        const uint8_t *r = _chr_c4(aa_ptr, (cc));\
        if (r) return r;\
    }

#define _unroll_one_chr_2(a_ptr, cc, loop_size, idx) {\
        const uint8_t *aa_ptr = (a_ptr) + ((loop_size) * (idx));\
        const uint8_t *r = _chr_c2(aa_ptr, (cc));\
        if (r) return r;\
    }

#define _unroll_one_chr_1(a_ptr, cc, loop_size, idx) {\
        const uint8_t *aa_ptr = (a_ptr) + ((loop_size) * (idx));\
        const uint8_t *r = _chr_c1(aa_ptr, (cc));\
        if (r) return r;\
    }


inline static const uint8_t *_memchr_remaining_15_bytes_impl(const uint8_t *buf_ptr, uint64_t cc64, const uint8_t *end_ptr)
{
    {
        const size_t loop_size = 8;
        const uint8_t * end_ptr_8 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_8) {
            _unroll_one_chr_8(buf_ptr, cc64, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    {
        const size_t loop_size = 4;
        const uint32_t cc32 = (uint32_t)cc64;
        const uint8_t * end_ptr_4 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_4) {
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    {
        const size_t loop_size = 2;
        const uint16_t cc = (uint16_t)cc64;
        const uint8_t * end_ptr_2 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_2) {
            _unroll_one_chr_2(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    {
        const size_t loop_size = 1;
        const uint8_t cc = (uint8_t)cc64;
        const uint8_t * end_ptr_1 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_1) {
            _unroll_one_chr_1(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    return (const uint8_t *)0;
}

inline static const uint8_t *_memchr_remaining_7_bytes_impl(const uint8_t *buf_ptr, uint32_t cc32, const uint8_t *end_ptr)
{
    {
        const size_t loop_size = 4;
        const uint8_t * end_ptr_4 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_4) {
            _unroll_one_chr_4(buf_ptr, cc32, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    {
        const size_t loop_size = 2;
        const uint16_t cc = (uint16_t)cc32;
        const uint8_t * end_ptr_2 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_2) {
            _unroll_one_chr_2(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    {
        const size_t loop_size = 1;
        const uint8_t cc = (uint8_t)cc32;
        const uint8_t * end_ptr_1 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_1) {
            _unroll_one_chr_1(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    return (const uint8_t *)0;
}

inline static const uint8_t *_memchr_remaining_3_bytes_impl(const uint8_t *buf_ptr, uint16_t cc16, const uint8_t *end_ptr)
{
    {
        const size_t loop_size = 2;
        const uint16_t cc = cc16;
        const uint8_t * end_ptr_2 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_2) {
            _unroll_one_chr_2(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    {
        const size_t loop_size = 1;
        const uint8_t cc = (uint8_t)cc16;
        const uint8_t * end_ptr_1 = end_ptr - loop_size;
        if (buf_ptr <= end_ptr_1) {
            _unroll_one_chr_1(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = buf_ptr + loop_size;
        }
    }
    return (const uint8_t *)0;
}
