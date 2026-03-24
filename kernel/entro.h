// entro.h
#ifndef ENTRO_H
#define ENTRO_H

#define SCALE 10000

struct freq_val { u16 count; };

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 256);
    __type(key, u32);
    __type(value, struct freq_val);
} freq_map SEC(".maps");

static __always_inline u32 calcular_entropia(u8 *buf, u32 size)
{
    u32 i;
    struct freq_val zero = {};

    #pragma unroll
    for (i = 0; i < 256; i++)
        bpf_map_update_elem(&freq_map, &i, &zero, BPF_ANY);

    if (size > 64) size = 64;
    u32 total = size;

    #pragma unroll
    for (i = 0; i < 64; i++) {
        if (i >= size) break;
        u8 byte = 0;
        bpf_probe_read_user(&byte, 1, &buf[i]);
        u32 key = byte;
        struct freq_val *v = bpf_map_lookup_elem(&freq_map, &key);
        if (v) v->count++;
    }

    if (total == 0) return 0;

    u32 entropia = 0;
    #pragma unroll
    for (i = 0; i < 256; i++) {
        struct freq_val *v = bpf_map_lookup_elem(&freq_map, &i);
        if (!v || v->count == 0) continue;
        u32 p = (u32)(((u64)v->count * SCALE) / total);
        if (p == 0) continue;
        u32 logp = 0;
        u32 tmp = p;
        #pragma unroll
        for (int j = 0; j < 14; j++) {
            if (tmp >= SCALE / 2) break;
            tmp <<= 1;
            logp += SCALE;
        }
        entropia += (u32)(((u64)p * logp) / SCALE);
    }
    return entropia;
}
#endif