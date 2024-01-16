#include "ptrace.h"

#include <sys/ptrace.h>
#include <unistd.h>
#include <sys/user.h>
#include <sys/uio.h>
#include <elf.h>

// MARK - Private

struct ptrace_syscall get_syscall_args(struct user_regs_struct *regs) {
    struct ptrace_syscall syscall;
    #if defined(__arm__)
        syscall.syscall = regs->ARM_r7;
        syscall.arg1 = regs->ARM_r0;
        syscall.arg2 = regs->ARM_r1;
        syscall.arg3 = regs->ARM_r2;
        syscall.arg4 = regs->ARM_r3;
        syscall.arg5 = regs->ARM_r4;
        syscall.arg6 = regs->ARM_r5;
    #elif defined(__aarch64__)
        syscall.syscall = regs->regs[8];
        syscall.arg1 = regs->regs[0];
        syscall.arg2 = regs->regs[1];
        syscall.arg3 = regs->regs[2];
        syscall.arg4 = regs->regs[3];
        syscall.arg5 = regs->regs[4];
        syscall.arg6 = regs->regs[5];
    #elif defined(__i386__)
        syscall.syscall = regs->orig_eax;
        syscall.arg1 = regs->ebx;
        syscall.arg2 = regs->ecx;
        syscall.arg3 = regs->edx;
        syscall.arg4 = regs->esi;
        syscall.arg5 = regs->edi;
        syscall.arg6 = regs->ebp;
    #elif defined(__x86_64__)
        syscall.syscall = regs->orig_rax;
        syscall.arg1 = regs->rdi;
        syscall.arg2 = regs->rsi;
        syscall.arg3 = regs->rdx;
        syscall.arg4 = regs->r10;
        syscall.arg5 = regs->r8;
        syscall.arg6 = regs->r9;
    #else
        #error "Unsupported architecture."
    #endif
    return syscall;
}

int get_syscall_result(struct user_regs_struct *regs) {
    return regs->regs[0];
}

// MARK: - Public

void ptrace_traceme() {
    ptrace(PTRACE_TRACEME, 0, NULL, NULL);
}

struct ptrace_syscall ptrace_get_syscall(pid_t pid) {
    struct user_regs_struct regs;
    struct iovec iov;
    iov.iov_base = &regs;
    iov.iov_len = sizeof(regs);

    ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &iov);
    return get_syscall_args(&regs);
}

int ptrace_get_syscall_results(pid_t pid) {
    struct user_regs_struct regs;
    struct iovec iov;
    iov.iov_base = &regs;
    iov.iov_len = sizeof(regs);

    ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &iov);
    return get_syscall_result(&regs);
}

void ptrace_continue_to_syscall(pid_t pid) {
    ptrace(PTRACE_SYSCALL, pid, NULL, NULL);
}
