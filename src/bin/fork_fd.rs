use nix::{sys::{wait::waitpid, stat::Mode},fcntl::{open, OFlag}, unistd::{fork, ForkResult, write, read}};

fn main() -> anyhow::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let fd = open(filename.as_str(), OFlag::O_RDONLY, Mode::empty())?;
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap();
            println!("Returned to parent - child is finished.");
            read_all("parent", fd)?;
        }
        Ok(ForkResult::Child) => {
            println!("Child started");
            read_all("child", fd)?;
            println!("Child finished");
        }
        Err(_) => println!("Fork failed"),
     }
     Ok(())
}

fn read_all(label: &str, fd: i32) -> anyhow::Result<()> {
    let mut bytes = [0; 100];
    let mut bytes_read = bytes.len();
    while bytes_read == bytes.len() {
        bytes_read = read(fd, &mut bytes)?;
        for i in bytes_read..bytes.len() {
            bytes[i] = 0;
        }
        println!("{label}: {}", std::str::from_utf8(&bytes)?);
    }
    Ok(())
}