use std::env::args;

use nix::{fcntl::{open, OFlag}, sys::stat::Mode, unistd::{read, close}};

fn main() {
    for arg in args().skip(1) {
        match dump(arg.as_str()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error when processing file {arg}: {e}");
            }
        }
    }
}

fn dump(filename: &str) -> anyhow::Result<()> {
    let fd = open(filename, OFlag::O_RDONLY, Mode::empty())?;
    println!("Opening {filename} with file descriptor {fd}");
    let mut bytes = [0; 100];
    let mut bytes_read = bytes.len();
    while bytes_read == bytes.len() {
        bytes_read = read(fd, &mut bytes)?;
        for i in bytes_read..bytes.len() {
            bytes[i] = 0;
        }
        print!("{}", std::str::from_utf8(&bytes)?);
    }
    println!();
    close(fd)?;
    Ok(())
}