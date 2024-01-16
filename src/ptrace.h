#ifndef PTRACE_H
#define PTRACE_H

#include <sys/types.h>

void ptrace_traceme();

struct ptrace_syscall {
    int syscall;
    int arg1;
    int arg2;
    int arg3;
    int arg4;
    int arg5;
    int arg6;
};

struct ptrace_syscall ptrace_get_syscall(pid_t pid);
int ptrace_get_syscall_results(pid_t pid);

void ptrace_continue_to_syscall(pid_t pid);

#endif // PTRACE_H
