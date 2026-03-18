#include <linux/types.h>
#include "vmlinux.h"
#define SCALE 10000

static __always_inline s32 log2_aprox(u32 p)
{
    if (p == 0) return 0;

    s32 exp = 0;
    #pragma unroll
    for (int i = 0; i < 14; i++) {
        if (p >= SCALE / 2) break;
        p <<= 1;
        exp--;
    }

    s32 frac = ((s32)p - (s32)(SCALE / 2)) * 2 - SCALE;
    return (exp * (s32)SCALE) + frac;
}

static __always_inline u32 calcular_entropia(u8 *buf, u32 size)
{
    u16 freq[256] = {};
    u32 total = 0;

    if (size > 256)
        size = 256;

    #pragma unroll
    for (int i = 0; i < 256; i++) {
        if (i >= (int)size) break;
        u8 byte = 0;
        bpf_probe_read_user(&byte, sizeof(byte), &buf[i]);
        freq[byte]++;
        total++;
    }

    if (total == 0)
        return 0;

    s32 entropia = 0;

    #pragma unroll
    for (int i = 0; i < 256; i++) {
        if (freq[i] == 0) continue;

        u32 p    = (u32)(((u64)freq[i] * SCALE) / total);
        if (p == 0) continue;

        s32 logp = log2_aprox(p);              // negativo
        s32 term = (s32)(((s64)p * logp) / SCALE); // negativo
        entropia -= term;                      // suma positiva
    }

    return (u32)entropia;  // ya es positivo
}