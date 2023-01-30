use std::{env::args, fs::File, io::{BufReader, BufRead}};

fn main() {
    for arg in args().skip(1) {
        match dump(arg.as_str()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error when processing file {arg}: {e}");
            }
        }
    }
}

fn dump(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    for (i, line) in buffer.lines().enumerate() {
        println!("{}: {}", i + 1, line?);
    }
    Ok(())
}