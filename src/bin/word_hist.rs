use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0..=1 => {
            println!("Usage: word_hist (-i|-a) filenames")
        }
        _ => {
            let use_all = match args[0].as_str() {
                "-i" => false,
                "-a" => true,
                _error => panic!("Illegal argument: {_error}")
            };
            match histograms(use_all, &args[1..]) {
                Ok(()) => {},
                Err(e) => println!("Error when reading files: {e}")
            }
        }
    }
}

fn histograms(use_all: bool, filenames: &[String]) -> anyhow::Result<()> {
    let mut histogram = HashMap::new();
    for filename in filenames {
        if !use_all {
            histogram.clear();
        }
        let f = File::open(filename)?;
        let buffer = BufReader::new(f);
        for line in buffer.lines() {
            let line = line?;
            for word in line.split_whitespace() {
                match histogram.get_mut(word) {
                    None => {
                        histogram.insert(word.to_owned(), 1);
                    }
                    Some(count) => {
                        *count += 1;
                    }
                }
            }
        }
        if !use_all {
            print_histogram(&histogram);
        }
    }
    if use_all {
        print_histogram(&histogram);
    }
    Ok(())
}

fn print_histogram(histogram: &HashMap<String,u64>) {
    let sorted = sorted_histogram(histogram);
    for (key, value) in sorted.iter() {
        println!("{key}: {value}");
    }
}

fn sorted_histogram(histogram: &HashMap<String,u64>) -> Vec<(&String,u64)> {
    let mut counts = Vec::new();
    for (key, value) in histogram.iter() {
        counts.push((key, *value));
    }
    counts.sort_by(|w1, w2| w2.1.cmp(&w1.1));
    counts
}