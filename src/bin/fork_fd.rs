use nix::{sys::{wait::waitpid, stat::Mode},fcntl::{open, OFlag}, unistd::{fork, ForkResult, read}};

fn main() -> anyhow::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let fd = open(filename.as_str(), OFlag::O_RDONLY, Mode::empty())?;
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap();
            println!("Returned to parent - child is finished.");
            read_some("parent", fd)?;
        }
        Ok(ForkResult::Child) => {
            println!("Child started");
            read_some("child", fd)?;
            println!("Child finished");
        }
        Err(_) => println!("Fork failed"),
     }
     Ok(())
}

fn read_some(label: &str, fd: i32) -> anyhow::Result<()> {
    let mut bytes = [0; 100];
    let bytes_read = read(fd, &mut bytes)?;
    for i in bytes_read..bytes.len() {
        bytes[i] = 0;
    }
    println!("{label}: {}", std::str::from_utf8(&bytes)?);
    Ok(())
}