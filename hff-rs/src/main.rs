use clap::Parser;
use serde_json;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a custom mapping file in *.hfc format
    #[arg(short, long)]
    mapping: Option<String>,
}

fn main() {
    let args = Args::parse();

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
        Ok(response) => match args.mapping {
            Some(mapping) => {
                println!(
                    "{}",
                    hff_rs::builder()
                        .with_file(&Path::new(&mapping))
                        .run(response)
                        .unwrap()
                );
            }
            None => {
                println!("{}", hff_rs::builder().run(response).unwrap());
            }
        },
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
        }
    }
}
