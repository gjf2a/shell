use std::{env::args, fs::File, io::Read};

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
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let bytes_read = file.read_to_string(&mut contents)?;
    println!("Read {bytes_read} bytes.");
    println!("{contents}");
    Ok(())
}