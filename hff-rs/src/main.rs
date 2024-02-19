use std::env;
use reqwest::header::HeaderMap;
use serde_json; // Add import statement for serde_json crate

use crate::hff::friendly;
mod hff;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Ensure there's a URL argument
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let token = match env::var("AUTH_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("AUTH_TOKEN environment variable not found.");
            std::process::exit(1);
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{}", friendly(response)?);

    Ok(())
}
