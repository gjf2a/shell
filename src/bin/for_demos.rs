fn main() {
    for i in 0..6 {
        print!("{i} ");
    }
    println!();
    for i in (0..6).rev() {
        print!("{i} ");
    }
    println!();
    for i in (1..6).rev() {
        print!("{i} ");
    }
    println!();
    for i in (0..6).step_by(2) {
        print!("{i} ");
    }
    println!();
    for i in (0..=6).step_by(2) {
        print!("{i} ");
    }
    println!();
    for i in (0..=6).rev().step_by(2) {
        print!("{i} ");
    }
    println!();
}