use nix::Result;
use nix::unistd::Pid;

use crate::ptrace_c;
pub use ptrace_c::ptrace_syscall;

pub fn traceme() -> Result<()> {
    // TODO: error handle
    unsafe { ptrace_c::ptrace_traceme() }
    Ok(())
}

pub fn ptrace_get_syscall(pid: Pid) -> Result<ptrace_syscall> {
    // TODO: error handle
    unsafe { Ok(ptrace_c::ptrace_get_syscall(pid.as_raw() as i32)) }
}

pub fn ptrace_get_syscall_results(pid: Pid) -> Result<i32> {
    // TODO: error handle
    unsafe { Ok(ptrace_c::ptrace_get_syscall_results(pid.as_raw() as i32)) }
}

pub fn ptrace_continue_to_syscall(pid: Pid) -> Result<()> {
    // TODO: error handle
    unsafe { ptrace_c::ptrace_continue_to_syscall(pid.as_raw() as i32) }
    Ok(())
}


