//#include <emmintrin.h>

#ifdef _MSC_VER
#  include <intrin.h>
#else
#  include <x86intrin.h>
#endif
#include "../../mem_rchr.h"


inline static const uint8_t *_rchr_c16_uu(const uint8_t *buf_ptr, __m128i mm_c16)
{
    __m128i mm_a = _mm_loadu_si128((const __m128i *)buf_ptr);
    __m128i mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    int mask = _mm_movemask_epi8(mm_eq);
    if (mask) {
        return buf_ptr + 16 - 1 - leading_zeros_u16((uint16_t)mask);
    } else {
        return (const uint8_t *)0;
    }
}

inline static const uint8_t *_rchr_c16_aa(const uint8_t *buf_ptr, __m128i mm_c16)
{
    __m128i mm_a = _mm_load_si128((const __m128i *)buf_ptr);
    __m128i mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    int mask = _mm_movemask_epi8(mm_eq);
    if (mask) {
        return buf_ptr + 16 - 1 - leading_zeros_u16((uint16_t)mask);
    } else {
        return (const uint8_t *)0;
    }
}

#define _unroll_one_rchr_16_uu(a_ptr, cc, loop_size, idx) \
    {\
        const uint8_t *aa_ptr = (a_ptr) + ((loop_size) * (idx));\
        const uint8_t *r = _rchr_c16_uu(aa_ptr, (cc));\
        if (r) {\
            return r;\
        }\
    }

#define _unroll_one_rchr_16_aa(a_ptr, cc, loop_size, idx) \
    {\
        const uint8_t *aa_ptr = (a_ptr) + ((loop_size) * (idx));\
        const uint8_t *r = _rchr_c16_aa(aa_ptr, (cc));\
        if (r) {\
            return r;\
        }\
    }

//
// _mm_load_si128()     CPI: 0.5
// _mm_cmpeq_epi8()     CPI: 0.5
// _mm_or_si128()       CPI: 0.33
// _mm_movemask_epi8()  CPI: 1.0

#ifdef __x86_64__
const uint8_t *_memrchr_sse2_impl(const uint8_t *s, size_t n, uint8_t c)
{
    const size_t len = n;
    const uint8_t * start_ptr = s;
    const uint8_t * buf_ptr_cur = start_ptr + len;
    //
    const uint64_t cc64 = (uint64_t)c * 0x0101010101010101LL;
    //
    if (len >= 16) {
        __m128i cc128 = _mm_set1_epi8((int8_t)c);
        {
            const size_t unroll = 4;
            const size_t loop_size = 4;
            const uint8_t * buf_ptr = buf_ptr_cur - (loop_size * unroll);
            size_t remaining_align = 0x10L - (((size_t)buf_ptr) & 0x0FL);
            if (buf_ptr >= start_ptr) {
                _unroll_one_rchr_4(buf_ptr, (uint32_t)cc64, loop_size, 3);
                _unroll_one_rchr_4(buf_ptr, (uint32_t)cc64, loop_size, 2);
                _unroll_one_rchr_4(buf_ptr, (uint32_t)cc64, loop_size, 1);
                _unroll_one_rchr_4(buf_ptr, (uint32_t)cc64, loop_size, 0);
            }
            /*
            const size_t loop_size = 16;
            const uint8_t * buf_ptr = buf_ptr_cur - loop_size;
            {
                const uint8_t *aa0_ptr = buf_ptr;
                __m128i mm_a0 = _mm_loadu_si128((const __m128i *)aa0_ptr);
                __m128i mm_eq0 = _mm_cmpeq_epi8(mm_a0, cc128);
                int mask0 = _mm_movemask_epi8(mm_eq0);
                if (mask0) return aa0_ptr + 16 - 1 - leading_zeros_u16((uint16_t)mask);
            }
            */
            //
            buf_ptr_cur = buf_ptr + remaining_align;
        }
        {
            const size_t unroll = 8;
            const size_t loop_size = 16;
            const uint8_t * buf_ptr = buf_ptr_cur;
            const uint8_t * min_ptr = start_ptr + (loop_size * unroll);
            while (buf_ptr >= min_ptr) {
                buf_ptr = buf_ptr - (loop_size * unroll);
                const uint8_t *aa0_ptr = buf_ptr + ((loop_size) * (0));
                const uint8_t *aa1_ptr = buf_ptr + ((loop_size) * (1));
                const uint8_t *aa2_ptr = buf_ptr + ((loop_size) * (2));
                const uint8_t *aa3_ptr = buf_ptr + ((loop_size) * (3));
                const uint8_t *aa4_ptr = buf_ptr + ((loop_size) * (4));
                const uint8_t *aa5_ptr = buf_ptr + ((loop_size) * (5));
                const uint8_t *aa6_ptr = buf_ptr + ((loop_size) * (6));
                const uint8_t *aa7_ptr = buf_ptr + ((loop_size) * (7));
                //
                __m128i mm_a0 = _mm_load_si128((const __m128i *)aa0_ptr);
                __m128i mm_a1 = _mm_load_si128((const __m128i *)aa1_ptr);
                __m128i mm_eq0 = _mm_cmpeq_epi8(mm_a0, cc128);
                __m128i mm_eq1 = _mm_cmpeq_epi8(mm_a1, cc128);
                __m128i mm_or01 = _mm_or_si128(mm_eq0, mm_eq1);
                //
                __m128i mm_a2 = _mm_load_si128((const __m128i *)aa2_ptr);
                __m128i mm_a3 = _mm_load_si128((const __m128i *)aa3_ptr);
                __m128i mm_eq2 = _mm_cmpeq_epi8(mm_a2, cc128);
                __m128i mm_eq3 = _mm_cmpeq_epi8(mm_a3, cc128);
                __m128i mm_or23 = _mm_or_si128(mm_eq2, mm_eq3);
                //
                __m128i mm_or03 = _mm_or_si128(mm_or01, mm_or23);
                //
                __m128i mm_a4 = _mm_load_si128((const __m128i *)aa4_ptr);
                __m128i mm_a5 = _mm_load_si128((const __m128i *)aa5_ptr);
                __m128i mm_eq4 = _mm_cmpeq_epi8(mm_a4, cc128);
                __m128i mm_eq5 = _mm_cmpeq_epi8(mm_a5, cc128);
                __m128i mm_or45 = _mm_or_si128(mm_eq4, mm_eq5);
                //
                __m128i mm_a6 = _mm_load_si128((const __m128i *)aa6_ptr);
                __m128i mm_a7 = _mm_load_si128((const __m128i *)aa7_ptr);
                __m128i mm_eq6 = _mm_cmpeq_epi8(mm_a6, cc128);
                __m128i mm_eq7 = _mm_cmpeq_epi8(mm_a7, cc128);
                __m128i mm_or67 = _mm_or_si128(mm_eq6, mm_eq7);
                //
                __m128i mm_or47 = _mm_or_si128(mm_or45, mm_or67);
                //
                __m128i mm_or07 = _mm_or_si128(mm_or03, mm_or47);
                if (_mm_movemask_epi8(mm_or07)) {
                    if (_mm_movemask_epi8(mm_or47)) {
                        if (_mm_movemask_epi8(mm_or67)) {
                            int mask7 = _mm_movemask_epi8(mm_eq7);
                            if (mask7) return aa7_ptr + 15 - leading_zeros_u16((uint16_t)mask7);
                            int mask6 = _mm_movemask_epi8(mm_eq6);
                            if (mask6) return aa6_ptr + 15 - leading_zeros_u16((uint16_t)mask6);
                        } else {
                            int mask5 = _mm_movemask_epi8(mm_eq5);
                            if (mask5) return aa5_ptr + 15 - leading_zeros_u16((uint16_t)mask5);
                            int mask4 = _mm_movemask_epi8(mm_eq4);
                            if (mask4) return aa4_ptr + 15 - leading_zeros_u16((uint16_t)mask4);
                        }
                    } else {
                        if (_mm_movemask_epi8(mm_or23)) {
                            int mask3 = _mm_movemask_epi8(mm_eq3);
                            if (mask3) return aa3_ptr + 15 - leading_zeros_u16((uint16_t)mask3);
                            int mask2 = _mm_movemask_epi8(mm_eq2);
                            if (mask2) return aa2_ptr + 15 - leading_zeros_u16((uint16_t)mask2);
                        } else {
                            int mask1 = _mm_movemask_epi8(mm_eq1);
                            if (mask1) return aa1_ptr + 15 - leading_zeros_u16((uint16_t)mask1);
                            int mask0 = _mm_movemask_epi8(mm_eq0);
                            if (mask0) return aa0_ptr + 15 - leading_zeros_u16((uint16_t)mask0);
                        }
                    }
                }
            }
            buf_ptr_cur = buf_ptr;
        }
        {
            const size_t loop_size = 16;
            const uint8_t * buf_ptr = buf_ptr_cur;
            const uint8_t * min_ptr = start_ptr + loop_size;
            while (buf_ptr >= min_ptr) {
                buf_ptr = buf_ptr - loop_size;
                {
                    const uint8_t *aa0_ptr = buf_ptr + ((loop_size) * (0));
                    __m128i mm_a0 = _mm_load_si128((const __m128i *)aa0_ptr);
                    __m128i mm_eq0 = _mm_cmpeq_epi8(mm_a0, cc128);
                    int mask0 = _mm_movemask_epi8(mm_eq0);
                    if (mask0) return aa0_ptr + 15 - leading_zeros_u16((uint16_t)mask0);
                }
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    //const uint64_t cc64 = (uint64_t)c * 0x0101010101010101LL;
    return _memrchr_remaining_15_bytes_impl(buf_ptr_cur, cc64, start_ptr);
}
#endif
