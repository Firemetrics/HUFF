use jsonpath_lib as jsonpath;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::path::Path;
extern crate regex;
use regex::Regex;

/**
 * Extract first element from JSONPath query.
 * This function is very quiet and does not report any errors but returns empty strings instead.
 * Todo: Add debug messages on errors.
 */
fn xjsonp_first(v: &serde_json::Value, json_path: &str) -> String {
    let mut selector = jsonpath::selector(v);
    match selector(json_path) {
        Ok(ret) => {
            if ret.len() == 1 {
                match ret[0].as_str() {
                    Some(s) => return s.to_string(),
                    None => match ret[0].as_array() {
                        Some(arr) => {
                            return arr
                                .iter()
                                .filter_map(|opt| opt.as_str())
                                .collect::<Vec<&str>>()
                                .join(" ");
                        }
                        None => match ret[0].as_number() {
                            Some(n) => return n.to_string(),
                            None => match ret[0].as_bool() {
                                Some(b) => return b.to_string(),
                                None => return "".to_string(),
                            },
                        },
                    },
                }
            } else if ret.len() > 1 {
                return ret
                    .iter()
                    .filter_map(|opt| opt.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ");
            }

            return "".to_string();
        }
        _ => {
            return "".to_string();
        }
    }
}

/**
 * Check if a JSON Path expression matches.
 */
/*
fn xjsonp_match(v: &serde_json::Value, json_path: &str) -> String {
    let mut selector = jsonpath::selector(v);
    match selector(json_path) {
        Ok(ret) => {
            if ret.len() == 1 {
                match ret[0].as_str() {
                    Some(s) => return s.to_string(),
                    None => match ret[0].as_array() {
                        Some(arr) => return arr.iter().filter_map(|opt| opt.as_str()).collect::<Vec<&str>>().join(" "),
                        None => match ret[0].as_number() {
                            Some(n) => return n.to_string(),
                            None => match ret[0].as_bool() {
                                Some(b) => return b.to_string(),
                                None => return "".to_string()
                            }
                        }
                    }
                }
            }
            else if ret.len() > 1 {
                return ret.iter().filter_map(|opt| opt.as_str()).collect::<Vec<&str>>().join(" ");
            }

            return "".to_string();
        }
        _ => {
            return "".to_string();
        }
    }
}
*/

fn parse_signature(input: &str) -> Result<Vec<String>, std::io::Error> {
    let pattern = Regex::new(r"^#\[(.*)\]$").unwrap();
    if let Some(captures) = pattern.captures(input) {
        let trimmed = captures.get(1).unwrap().as_str(); // Get inner contents
        Ok(trimmed.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            format!("Invalid signature format: {}", input),
        ))
    }
}

pub fn apply_format(v: &serde_json::Value, input: &str) -> String {
    let re = Regex::new(r"\{(\$.+?)\}").expect("Failed to compile regex");
    return re
        .replace_all(input, |caps: &regex::Captures| xjsonp_first(v, &caps[1]))
        .to_string();
}

pub fn signature_to_str(signature: Vec<String>) -> String {
    // sort and join signature
    let mut sorted_signature = signature.clone();
    sorted_signature.sort();
    return sorted_signature.join("|");
}

pub fn load_default_mapping() -> io::Result<Vec<String>> {
    load_mapping_from_str(default_mapping())
}

pub fn process_mapping(mapping: &Vec<String>) -> Result<HashMap<String, String>, std::io::Error> {
    let mut mappers = HashMap::new();
    for pair in mapping.chunks(2) {
        match pair {
            [signature_str, format_str] => {
                if let Ok(parsed_signature) = parse_signature(signature_str.trim()) {
                    mappers.insert(
                        signature_to_str(parsed_signature),
                        format_str.trim().to_string(),
                    );
                } else {
                    return Err(std::io::Error::new(
                        ErrorKind::InvalidData,
                        "Failed to parse signature",
                    ));
                }
            }
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid format: unequal number of lines in mapping file",
                ));
            }
        }
    }

    Ok(mappers)
}

pub fn load_mapping_from_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let filtered_lines = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim_start().starts_with("//")) // remove comments
        .collect::<Vec<String>>();
    return Ok(filtered_lines);
}

pub fn load_mapping_from_str(mapping_str: &str) -> io::Result<Vec<String>> {
    let filtered_lines = mapping_str
        .lines()
        .filter(|line| !line.trim_start().starts_with("//")) // remove comments
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| line.to_owned())
        .collect();
    return Ok(filtered_lines);
}

pub fn default_mapping() -> &'static str {
    include_str!("../../resources/mapping.hfc")
}
