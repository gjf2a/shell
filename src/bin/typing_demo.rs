use std::io::Write;

fn main() {
    loop {
        match process_line() {
            Ok(keep_going) => {
                if !keep_going {
                    break;
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

fn process_line() -> anyhow::Result<bool> {
    let mut user_input = String::new();
    print!("$ ");
    std::io::stdout().flush()?; 
    std::io::stdin().read_line(&mut user_input)?;
    if user_input.trim() == "exit" {
        Ok(false)
    } else {
        let words: Vec<&str> = user_input.split_whitespace().collect();
        println!("You entered: {words:?}");
        Ok(true)
    }
}