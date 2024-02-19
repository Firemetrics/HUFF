use std::io::{self, BufRead};
use serde_json; // Add import statement for serde_json crate

use crate::hff::friendly;
mod hff;

fn main() {
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buffer = String::new();
    loop {
        match handle.read_line(&mut buffer) {
            Ok(0) => break, // End of stream
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break; // Stop on error
            }
        }
    }

    println!("{}", buffer);

    match serde_json::from_str(&buffer) {
        Ok(response) => {
            println!("{}", friendly(response).unwrap());
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
        }
    }
}
