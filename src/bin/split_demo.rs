fn main() {
    for arg in std::env::args().skip(1) {
        println!("Splitting {arg}");
        let parts = splitter(arg.as_str(), ':');
        for part in parts.iter() {
            println!("{part}");
        }
    }
}

fn splitter(s: &str, spl: char) -> Vec<String> {
    let mut owned = vec![];
    for part in s.split(spl) {
        owned.push(part.to_owned());
    }
    owned
}