fn main() {
    let mut sum = 0;
    for arg in std::env::args().skip(1) {
        sum += arg.parse::<i32>().unwrap();
        /*match arg.parse::<i32>() {
            Ok(n) => {sum += n;}
            Err(e) => {println!("Error: {e}");}
        }*/
    }
    println!("Sum: {sum}");
}