use nix::unistd::{fork, ForkResult};

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            busy_work("parent");
        }
        Ok(ForkResult::Child) => {
            println!("Child process starting");
            busy_work("child");
        }
        Err(_) => println!("Fork failed"),
     }
}

fn busy_work(label: &str) {
    let mut counter: u128 = 0;
    loop {
        counter += 1;
        if counter % 100000 == 0 {
            println!("{label}");
        }
    }
}