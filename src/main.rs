use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execvp};
use nix::Result;
use std::ffi::CString;
use nix::sys::signal::{Signal, raise};

mod ptrace_c;
mod ptrace;

fn main() -> Result<()> {
    match unsafe { fork()? } {
        ForkResult::Parent { child } => {
            let mut in_syscall = false;
            loop {
                if let Ok(WaitStatus::Exited(_, _)) = waitpid(child, None) {
                    break;
                }

                if in_syscall {
                    let syscall = ptrace::ptrace_get_syscall(child)?;
                    println!("ptrace_syscall return value: {}", syscall.syscall);
                    in_syscall = false;
                } else {
                    let result = ptrace::ptrace_get_syscall_results(child)?;
                    println!("ptrace_syscall number: {}", result);
                    in_syscall = true;
                }

                ptrace::ptrace_continue_to_syscall(child)?;
            }
        }
        ForkResult::Child => {
            ptrace::traceme()?;
            let path = CString::new("/bin/ls").unwrap();
            let arg = CString::new("ls").unwrap();
            raise(Signal::SIGSTOP)?; // Add this line to stop the child process
            execvp(&path, &[arg])?;
        }
    }
    Ok(())
}
