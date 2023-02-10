use std::ops::Add;

fn main() {
    let mut values = vec![];
    for arg in std::env::args().skip(1) {
        values.push(NumStringValue::from_string(arg.as_str()));
    }
    println!("{:?}", values[0].clone() + values[1].clone());
    println!("{:?}", values[0].clone() + values[2].clone());
    values.sort();
    for v in values.iter() {
        println!("{v:?}");
    }

}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum NumStringValue {
    Num(i64),
    Str(String)
}

impl Add for NumStringValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            NumStringValue::Num(n1) => match &rhs {
                NumStringValue::Num(n2) => Self::Num(*n1 + *n2),
                NumStringValue::Str(s) => Self::Str(format!("{n1}{s}")),
            }
            NumStringValue::Str(s1) => match &rhs {
                NumStringValue::Num(n) => Self::Str(format!("{s1}{n}")),
                NumStringValue::Str(s2) => Self::Str(format!("{s1}{s2}")),
            }
        }
    }
}

impl NumStringValue {
    fn from_string(s: &str) -> Self {
        match s.parse::<i64>() {
            Ok(n) => Self::Num(n),
            Err(_) => Self::Str(s.to_owned()),
        }
    }
}