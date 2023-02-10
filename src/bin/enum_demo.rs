fn main() {
    let mut values = std::env::args().skip(1).map(|a| NumWordValue::new(a.as_str())).collect::<Vec<_>>();
    values.sort();
    println!("{values:?}");
}

#[derive(Debug, Eq, PartialEq, Ord)]
enum NumWordValue {
    Num(i64),
    Word(String)
}

impl NumWordValue {
    fn new(input: &str) -> Self {
        match input.parse::<i64>() {
            Ok(n) => Self::Num(n),
            Err(_) => Self::Word(input.to_string()),
        }
    }
}

impl PartialOrd for NumWordValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            NumWordValue::Num(n1) => match other {
                NumWordValue::Num(n2) => n1.partial_cmp(n2),
                NumWordValue::Word(_) => Some(std::cmp::Ordering::Less),
            }
            NumWordValue::Word(w1) => match other {
                NumWordValue::Num(_) => Some(std::cmp::Ordering::Greater),
                NumWordValue::Word(w2) => w1.partial_cmp(w2),
            }
        }
    }
}