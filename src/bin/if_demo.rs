fn main() {
    let mut sum = 0;
    for arg in std::env::args().skip(1) {
        sum += arg.parse::<i32>().unwrap();
    }
    let v = if sum < 0 {"Negative"} else if sum > 0 {"Positive"} else {"Zero"};
    println!("{v}");
}