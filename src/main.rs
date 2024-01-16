use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execvp};
use nix::Result;
use std::ffi::CString;
use nix::sys::signal::{Signal, raise};
use serde_json;
use serde::Serialize;

mod ptrace_c;
mod ptrace;

#[derive(Serialize)]
struct SyscallOutput {
    id: u64,
    syscall: u64,
    args: Vec<u64>,
}

impl SyscallOutput {
    fn from_syscall(id: u64, syscall: &ptrace::ptrace_syscall) -> SyscallOutput {
        SyscallOutput {
            id: id,
            syscall: syscall.syscall as u64,
            args: vec![
                syscall.arg1 as u64,
                syscall.arg2 as u64,
                syscall.arg3 as u64,
                syscall.arg4 as u64,
                syscall.arg5 as u64,
                syscall.arg6 as u64,
            ],
        }
    }
}

#[derive(Serialize)]
struct SyscallResultOutput {
    id: u64,
    result: u64,
}

impl SyscallResultOutput {
    fn from_syscall_result(id: u64, result: &i32) -> SyscallResultOutput {
        SyscallResultOutput {
            id: id,
            result: *result as u64,
        }
    }
}


fn main() -> Result<()> {
    match unsafe { fork()? } {
        ForkResult::Parent { child } => {
            let mut id = 0;
            let mut in_syscall = false;

            loop {
                if let Ok(WaitStatus::Exited(_, _)) = waitpid(child, None) {
                    break;
                }

                if in_syscall {
                    // Print results
                    let result = ptrace::ptrace_get_syscall_results(child)?;
                    let output = SyscallResultOutput::from_syscall_result(id, &result);
                    let json = serde_json::to_string(&output).unwrap();
                    println!("{}", json);
                    id += 1;
                    in_syscall = false;
                } else {
                    // Print args
                    let syscall = ptrace::ptrace_get_syscall(child)?;
                    let output = SyscallOutput::from_syscall(id, &syscall);
                    let json = serde_json::to_string(&output).unwrap();
                    println!("{}", json);
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
