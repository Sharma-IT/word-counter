use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        read_from_files(&args[1..])
    } else {
        read_from_stdin()
    }?;

    if input.is_empty() {
        println!("No input provided.");
        return Ok(());
    }

    let total_words = count_words(&input);
    println!("\nTotal word count: {}", total_words);

    Ok(())
}

fn read_from_files(files: &[String]) -> Result<String, io::Error> {
    let mut input = String::new();
    for file in files {
        let content = fs::read_to_string(file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Error reading file {}: {}", file, e)))?;
        input.push_str(&content);
        input.push('\n');
    }
    Ok(input.trim_end().to_string())
}

fn read_from_stdin() -> Result<String, io::Error> {
    println!("Enter text (press <ENTER> and Ctrl + D or press Ctrl + D three times to end input):");
    let stdin = io::stdin();
    let mut input = String::new();
    for line in stdin.lock().lines() {
        input.push_str(&line?);
        input.push(' ');
    }
    Ok(input.trim_end().to_string())
}

fn count_words(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| {
            let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            !cleaned_word.is_empty()
        })
        .count()
}
