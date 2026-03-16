#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include "vmlinux.h"

char LICENSE[] SEC("license") = "GPL";

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_open(struct trace_event_raw_sys_enter *ctx){
    u32 pid;
    char comm[16];
    pid = bpf_get_current_pid_tgid() >> 32;
    bpf_get_current_comm(&comm, sizeof(comm));
    bpf_printk("openat called by PID=%d COMM=%s\n", pid, comm);
    return 0;
}