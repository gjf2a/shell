use nix::unistd::{fork, ForkResult};

fn main() {
    let mut nums: Vec<i64> = (0..10).collect();
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            process_nums("parent", &mut nums, 1);
            println!("Returned to parent - child is finished.");
        }
        Ok(ForkResult::Child) => {
            println!("Child process starting");
            process_nums("child", &mut nums, 2);
            println!("Child process finished");
        }
        Err(_) => println!("Fork failed"),
     }
}

fn process_nums(label: &str, nums: &mut Vec<i64>, addend: i64) {
    for n in nums.iter_mut() {
        *n += addend;
    }
    for n in nums.iter() {
        println!("{label}:{n}");
        nix::unistd::sleep(1);
    }
}