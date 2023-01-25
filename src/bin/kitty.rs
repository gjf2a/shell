use std::{env::args, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    for arg in args().skip(1) {
        dump_unwrap(arg.as_str());
        match dump(arg.as_str()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error when processing file {arg}: {e}");
            }
        }
    }
    Ok(())
}

fn dump(filename: &str) -> std::io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let bytes_read = file.read_to_string(&mut contents)?;
    println!("Read {bytes_read} bytes.");
    println!("{contents}");
    Ok(())
}

fn dump_unwrap(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    let bytes_read = file.read_to_string(&mut contents).unwrap();
    println!("Read {bytes_read} bytes.");
    println!("{contents}");
}