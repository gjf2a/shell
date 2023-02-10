use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord)]
pub struct Coord {
    pub x: i64,
    pub y: i64
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let other_sum = other.x + other.y;
        (self.x + self.y).partial_cmp(&other_sum)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}