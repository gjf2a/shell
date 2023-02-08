use std::ffi::CString;

use nix::{unistd::{fork, pipe, close, dup2, execvp}, sys::wait::waitpid};

fn main() -> anyhow::Result<()> {
    println!("Execute ls -l | wc");

    let ls: Vec<CString> = vec![CString::new("ls")?, CString::new("-l")?];
    let wc: Vec<CString> = vec![CString::new("wc")?];

    match unsafe {fork()}? {
        nix::unistd::ForkResult::Parent { child } => {
            println!("wc pid is {child}");
            waitpid(child, None).unwrap();
            println!("Finished!");
        },
        nix::unistd::ForkResult::Child => {
            let (wc_in, ls_out) = pipe()?;
            match unsafe {fork()}? {
                nix::unistd::ForkResult::Parent { child: _ } => {
                    close(ls_out)?;
                    dup2(wc_in, 0)?;
                    execvp(&wc[0], &wc)?;
                }
                nix::unistd::ForkResult::Child => {
                    close(wc_in)?;
                    dup2(ls_out, 1)?;
                    execvp(&ls[0], &ls)?;
                }
            }
        }
    }
    Ok(())
}