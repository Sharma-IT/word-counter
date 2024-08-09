use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() > 1 {
        // Read from files specified as command-line arguments
        for arg in &args[1..] {
            match fs::read_to_string(arg) {
                Ok(content)x => {
                    input.push_str(&content);
                    input.push('\n');
                }
                Err(e) => eprintln!("Error reading file {}: {}", arg, e),
            }
        }
    } else {
        // Read from standard input
        println!("Enter text (press <ENTER> and Ctrl + D or press Ctrl + D three times to end input):");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    input.push_str(&line);
                    input.push(' '); // Add a space to separate lines
                }
                Err(_) => break, // Break on error (e.g., EOF)
            }
        }
    }

    // Remove the trailing space if present
    if input.ends_with(' ') {
        input.pop();
    }

    if !input.is_empty() {
        let total_words = count_words(&input);
        println!("\nTotal word count: {}", total_words);
    } else {
        println!("No input provided.");
    }
}

fn count_words(text: &str) -> usize {
    let mut total_words = 0;

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !word.is_empty() {
            total_words += 1;
        }
    }

    total_words
}