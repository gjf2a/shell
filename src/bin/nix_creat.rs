use nix::{fcntl::{OFlag, open}, sys::stat::Mode, unistd::{close, write}};

fn main() -> anyhow::Result<()> {
    let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
    let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
    let fd = open("foo", flags, mode)?;
    let text = "Hello, world!";
    let written = write(fd, text.as_bytes())?;
    close(fd)?;
    println!("Wrote {written} bytes to file descriptor {fd}");
    Ok(())
}