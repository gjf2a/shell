use nix::{fcntl::{OFlag, open}, sys::stat::Mode, unistd::{close, write}};

fn main() -> anyhow::Result<()> {
    let mut flags = OFlag::empty();
    for f in [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC] {
        flags.set(f, true);
    }
    let mut mode = Mode::all();
    for m in [Mode::S_IXGRP, Mode::S_IXOTH, Mode::S_IXUSR] {
        mode.set(m, false);
    }
    let fd = open("test.txt", flags, mode)?;
    let text = "Hello, world!";
    let written = write(fd, text.as_bytes())?;
    close(fd)?;
    println!("Wrote {written} bytes");
    Ok(())
}