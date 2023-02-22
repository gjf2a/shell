use std::{thread, sync::{Mutex, Arc}};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        println!("Usage: thread_demo num_threads num_consecutive");
    } else {
        let num_threads = args[0].parse::<usize>().unwrap();
        let num_consecutive = args[1].parse::<usize>().unwrap();
        let counter = Arc::new(Mutex::new(0));
        for _ in 0..num_threads {
            let counter = counter.clone();
            thread::spawn(move || {
                simulate(num_consecutive);
                let mut counter = counter.lock().unwrap();
                *counter += 1;
            });
        }

        loop {
            let counter = counter.lock().unwrap();
            if *counter == num_threads {
                break;
            }
        }
    }
}

fn simulate(target_flips: usize) {
    let mut consecutive = 0;
    let mut iterations = 0;
    while consecutive < target_flips {
        iterations += 1;
        if rand::random() {
            consecutive += 1;
        } else {
            consecutive = 0;
        }
    }
    println!("iterations: {iterations}");
}