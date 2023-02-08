

fn main() {
    let mut types = ArgTypes::new();
    for arg in std::env::args() {
        types.add(arg.as_str());
    }
    println!("{types:?}");
    println!("{}", types.sum());
    types.print_slashed();
}

#[derive(Debug)]
struct ArgTypes {
    nums: Vec<i64>,
    strings: Vec<String>,
    slash_string: Option<String>
}

impl ArgTypes {
    fn new() -> Self {
        ArgTypes {nums: vec![], strings: vec![], slash_string: None}
    }

    fn add(&mut self, arg: &str) {
        match arg.parse::<i64>() {
            Ok(n) => {
                self.nums.push(n);
            }
            Err(_) => {
                if arg.contains('/') && self.slash_string.is_none() {
                    self.slash_string = Some(arg.to_string());
                }
                self.strings.push(arg.to_string());
            }
        }
    }

    fn sum(&self) -> i64 {
        self.nums.iter().sum()
    }

    fn print_slashed(&self) {
        match &self.slash_string {
            Some(s) => {println!("{s}");}
            None => {println!("Nothing there");}
        }
    }
}