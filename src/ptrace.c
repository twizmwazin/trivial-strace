#include "ptrace.h"

#include <sys/ptrace.h>
#include <unistd.h>
#include <sys/user.h>
#include <sys/uio.h>
#include <elf.h>

// MARK - Private

struct ptrace_syscall get_syscall_args(struct user_regs_struct *regs) {
    struct ptrace_syscall syscall;
    // TODO: This only works on aarch64. Need to make it work on other architectures.
    syscall.syscall = regs->regs[8];
    syscall.arg1 = regs->regs[0];
    syscall.arg2 = regs->regs[1];
    syscall.arg3 = regs->regs[2];
    syscall.arg4 = regs->regs[3];
    syscall.arg5 = regs->regs[4];
    syscall.arg6 = regs->regs[5];
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
