use std::io::{self, BufRead};
use serde_json; 

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

    match serde_json::from_str(&buffer) {
        Ok(response) => {
            println!("{}", hff::builder().run(response).unwrap());
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
        }
    }
}
