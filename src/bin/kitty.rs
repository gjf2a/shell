use std::{env::args, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    for arg in args().skip(1) {
        dump(arg)?;
    }
    Ok(())
}

fn dump(filename: String) -> std::io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{contents}");
    Ok(())
}