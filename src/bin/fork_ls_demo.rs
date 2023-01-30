use std::ffi::CString;

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap();
            println!("Returned to parent - child is finished.");
        }
        Ok(ForkResult::Child) => {
            let cmd = externalize("ls");
            match execvp::<CString>(cmd[0].as_c_str(), &cmd) {
                Ok(_) => {},
                Err(e) => {println!("Could not execute: {e}");},
            }
        }
        Err(_) => println!("Fork failed"),
     }
}

fn externalize(command: &str) -> Box<[CString]> {
    let converted = command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    converted.into_boxed_slice()
}