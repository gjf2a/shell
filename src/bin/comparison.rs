use shell::Coord;

fn main() {
    let c1 = Coord {x: 4, y: 5};
    let c2 = Coord {x: 3, y: 5};
    let c3 = c1;
    println!("{c1:?}");
    println!("{c1}");
    println!("{}", c1 < c2);
    println!("{}", c1 > c2);
    println!("{}", c1 == c2);
    println!("{}", c1 == c1);
}



#[derive(PartialEq)]
struct FloatCoord {
    x: f64,
    y: f64
}