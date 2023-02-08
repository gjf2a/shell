fn main() {
    match read_write() {
        Ok(()) => {} 
        Err(e) => {println!("Error: {e}");}
    }
}

fn read_write() -> anyhow::Result<()> {
    loop {
        let mut bytes = [0; 1];
        let bytes_read = nix::unistd::read(0, &mut bytes)?;
        let bytes_out = format!("{bytes_read}");
        nix::unistd::write(2, bytes_out.as_bytes())?;
        if bytes_read == 0 {
            break;
        }
        nix::unistd::write(1, &bytes)?;
    }
    Ok(())
}