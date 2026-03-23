#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include "glrd_event.h"
#include "entro.h"
#define O_CREAT 0x00000100

char LICENSE[] SEC("license") = "GPL";

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));
} GLRD_EVENTS SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, struct glrd_event);
} event_scratch SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_writev")
int trace_writev(struct trace_event_raw_sys_enter *ctx) {
    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;

    struct iovec *iov = (struct iovec *)ctx->args[1];
    struct iovec iov0;
    bpf_probe_read_user(&iov0, sizeof(iov0), iov);

    char buf[256];
    bpf_probe_read_user(buf, sizeof(buf), iov0.iov_base);

    u32 entropia = calcular_entropia((u8 *)iov0.iov_base, iov0.iov_len);

    u32 key = 0;
    struct glrd_event *event = bpf_map_lookup_elem(&event_scratch, &key);
    if (!event) return 0;

    event->pid = pid;
    event->entropia_x10000 = entropia;
    event->timestamp_ns = bpf_ktime_get_ns();
    bpf_get_current_comm(event->nombre_proceso, sizeof(event->nombre_proceso));

    // una sola llamada, con event (puntero) no &event (puntero al puntero)
    bpf_perf_event_output(ctx, &GLRD_EVENTS, BPF_F_CURRENT_CPU,
                          event, sizeof(*event));
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(struct trace_event_raw_sys_enter *ctx) {
    int flags = (int)ctx->args[2];
    if (!(flags & O_CREAT)) return 0;

    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;

    char comm[16];
    bpf_get_current_comm(comm, sizeof(comm));

    char path[256];
    bpf_probe_read_user_str(path, sizeof(path), (const char *)ctx->args[1]);

    bpf_printk("O_CREAT: pid=%d comm=%s path=%s\n", pid, comm, path);
    return 0;
}