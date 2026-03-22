#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include "vmlinux.h"
#define O_CREAT 0x00000100

char LICENSE[] SEC("license") = "GPL";

SEC("tracepoint/syscalls/sys_enter_writev")
int trace_write(struct trace_event_raw_sys_enter *ctx){
    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;

    u64 uid_gid = bpf_get_current_uid_gid();
    u32 uid = uid_gid & 0xFFFFFFFF;
    char comm[16];
    bpf_get_current_comm(comm, sizeof(comm));

    int fd = (int)ctx->args[0];
    struct iovec *iov = (struct iovec *)ctx->args[1];
    u64 iovcnt = ctx->args[2];

    struct iovec iov0;
    bpf_probe_read_user(&iov0, sizeof(iov0), iov);

    char buff[256];
    bpf_probe_read_user(buff, sizeof(buff), iov0.iov_base);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(struct trace_event_raw_sys_enter *ctx){
    int flags = (int)ctx->args[2];
    if(!(flags & O_CREAT)) return 0;

    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;

    char comm[16];
    bpf_get_current_comm(comm, sizeof(comm));

    char path[256];
    bpf_probe_read_user_str(path, sizeof(path), (const char *)ctx->args[1]);
    bpf_printk("O_CREAT detectado: pid=%d comm=%s path=%s\n", pid, comm, path);

    return 0;
}