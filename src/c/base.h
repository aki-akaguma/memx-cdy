#include <string.h>
#include <stdint.h>

inline static size_t trailing_zeros_u64(uint64_t bits)
{
    if (!bits) return 64;
#ifdef __GNUC__
    return __builtin_ctzll(bits);
#else
#ifdef _MSC_VER
    unsigned long index;
    _BitScanForward64(&index, bits);
    return index;
#else
#error Undefined BSF
#endif
#endif
}

inline static size_t trailing_zeros_u32(uint32_t bits)
{
    if (!bits) return 32;
#ifdef __GNUC__
    return __builtin_ctz(bits);
#else
#ifdef _MSC_VER
    unsigned long index;
    _BitScanForward(&index, bits);
    return index;
#else
#error Undefined BSF
#endif
#endif
}

inline static size_t trailing_zeros_u16(uint16_t bits)
{
    if (!bits) return 16;
#ifdef __GNUC__
    return __builtin_ctz((uint32_t)bits);
#else
#ifdef _MSC_VER
    unsigned long index;
    _BitScanForward(&index, (uint32_t)bits);
    return index;
#else
#error Undefined BSF
#endif
#endif
}

inline static size_t leading_zeros_u64(uint64_t bits)
{
    if (!bits) return 64;
#ifdef __GNUC__
    return __builtin_clzll(bits);
#else
#ifdef _MSC_VER
    unsigned long index;
    _BitScanReverse64(&index, bits);
    return 63 - index;
#else
#error Undefined BSF
#endif
#endif
}

inline static size_t leading_zeros_u32(uint32_t bits)
{
    if (!bits) return 32;
#ifdef __GNUC__
    return __builtin_clz(bits);
#else
#ifdef _MSC_VER
    unsigned long index;
    _BitScanReverse(&index, bits);
    return 31 - index;
#else
#error Undefined BSF
#endif
#endif
}

inline static size_t leading_zeros_u16(uint16_t bits)
{
    if (!bits) return 16;
#ifdef __GNUC__
    return __builtin_clz((uint32_t)bits) - 16;
#else
#ifdef _MSC_VER
    unsigned long index;
    _BitScanReverse(&index, (uint32_t)bits);
    return 31 - index - 16;
#else
#error Undefined BSF
#endif
#endif
}
