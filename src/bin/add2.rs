
fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() != 2 {
        println!("Usage: add2 num1 num2");
    } else {
        match add(arguments[0].as_str(), arguments[1].as_str()) {
            Ok(sum) => {
                println!("{sum}"); 
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
        
    }
}

fn add(num1: &str, num2: &str) -> anyhow::Result<f64> {
    Ok(num1.parse::<f64>()? + num2.parse::<f64>()?)
}