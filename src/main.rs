use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execvp};
use nix::Result;
use std::ffi::CString;
use nix::sys::signal::{Signal, raise};
use serde_json;
use serde::Serialize;
use std::io::{Write, stderr};
use clap::Parser;
use std::fs::File;

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

fn do_trace<W>(output_file: &mut W, command: Vec<CString>) -> Result<()> where W: Write {
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
                    serde_json::to_writer(&mut *output_file, &output).unwrap();
                    output_file.write_all(b"\n").unwrap();
                    output_file.flush().unwrap();
                    id += 1;
                    in_syscall = false;
                } else {
                    // Print args
                    let syscall = ptrace::ptrace_get_syscall(child)?;
                    let output = SyscallOutput::from_syscall(id, &syscall);
                    serde_json::to_writer(&mut *output_file, &output).unwrap();
                    output_file.write_all(b"\n").unwrap();
                    output_file.flush().unwrap();
                    in_syscall = true;
                }

                ptrace::ptrace_continue_to_syscall(child)?;
            }
        }
        ForkResult::Child => {
            ptrace::traceme()?;
            raise(Signal::SIGSTOP)?;
            execvp(&command[0], &command.as_slice())?;
        }
    }
    Ok(())
}


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    output: Option<String>,

    #[arg(required = true)]
    command: Vec<String>,
}


fn main() -> Result<()> {
    let args = Args::parse();
    let converted_cmd: Vec<CString> = args.command.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();

    match args.output {
        Some(path) => {
            let mut output_file = File::create(path).expect("Failed to open output file");
            do_trace(&mut output_file, converted_cmd)
        },
        None => {
            do_trace(&mut stderr(), converted_cmd)
        },
    }.expect("Failed to trace");

    Ok(())
}
