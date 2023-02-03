use nix::{fcntl::{OFlag, open}, sys::stat::Mode, unistd::{close, write}};

fn main() -> anyhow::Result<()> {
    let mut flags = OFlag::empty();
    flags.set(OFlag::O_CREAT, true);
    flags.set(OFlag::O_WRONLY, true);
    flags.set(OFlag::O_TRUNC, true);
    let mut mode = Mode::empty();
    mode.set(Mode::S_IRUSR, true);
    mode.set(Mode::S_IWUSR, true);
    let fd = open("test.txt", flags, mode)?;
    let text = "Hello, world!";
    let written = write(fd, text.as_bytes())?;
    close(fd)?;
    println!("Wrote {written} bytes");
    Ok(())
}