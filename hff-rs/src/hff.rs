use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::collections::HashMap;
//use std::path::Path;
use serde_json;
use serde_json::json;
use jsonpath_lib as jsonpath;
use serde_yaml;
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

/**
 * Check if a JSON Path expression matches.
 */
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

fn parse_signature(input: &str) -> Result<Vec<String>, std::io::Error> {
    
    let pattern = Regex::new(r"^#\[(.*)\]$").unwrap();
    if let Some(captures) = pattern.captures(input) {
        let trimmed = captures.get(1).unwrap().as_str(); // Get inner contents
        Ok(trimmed.split(',')
                  .map(|s| s.trim().to_string())
                  .collect())
    } else {
        Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Invalid signature format: {}", input)))
    }
}

fn apply_format(v: &serde_json::Value, input: &str) -> String {
    let re = Regex::new(r"\{(\$.+?)\}").expect("Failed to compile regex");
    return re.replace_all(input, |caps: &regex::Captures| xjsonp_first(v, &caps[1])).to_string()
}

fn signature_to_str(signature: Vec<String>) -> String {
    // sort and join signature
    let mut sorted_signature = signature.clone();
    sorted_signature.sort();
    return sorted_signature.join("|");
}

fn load_mapping_from_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let filtered_lines = reader.lines()
                            .filter_map(Result::ok)
                            .filter(|line| !line.trim_start().starts_with("//")) // remove comments
                            .collect::<Vec<String>>();
    return Ok(filtered_lines);
}

fn load_mapping_from_str(mapping_str: &str) -> io::Result<Vec<String>> {
    let filtered_lines = mapping_str.lines()
                            .filter(|line| !line.trim_start().starts_with("//")) // remove comments
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|&line| line.to_owned())
                            .collect();
    return Ok(filtered_lines);
}

pub fn default_mapping() -> &'static str {
    include_str!("../resources/mapping.hfc")
}

fn load_default_mapping() -> io::Result<Vec<String>> {
    load_mapping_from_str(default_mapping())
}

fn process_mapping(mapping: &Vec<String>) -> Result<HashMap<String, String>, std::io::Error> {
    let mut mappers = HashMap::new();
    for pair in mapping.chunks(2) {
        match pair {
            [signature_str, format_str] => {
                if let Ok(parsed_signature) = parse_signature(signature_str.trim()) {
                    mappers.insert(
                        signature_to_str(parsed_signature), 
                        format_str.trim().to_string()
                    );
                } else {
                    return Err(std::io::Error::new(ErrorKind::InvalidData, "Failed to parse signature"));
                }
            }
            _ => return Err(std::io::Error::new(ErrorKind::InvalidData, "Invalid format: unequal number of lines in mapping file"))
        }
    }

    Ok(mappers)
}

/**
 * Recurse over JSON tree and pass branches to reformatting function.
 */
fn reformat_fhir(v: &serde_json::Value, k: Option<&str>, _formatters: &HashMap<String, String>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match k {
        // First-pass of JSON structure
        None => {
            if let Some(obj) = v.as_object() {
                let reformatted_obj: serde_json::Map<std::string::String, serde_json::Value> = obj
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), reformat_fhir(v, Some(k), _formatters)?)))
                    .collect::<Result<_, Box<dyn std::error::Error>>>()?;
                Ok(serde_json::Value::Object(reformatted_obj))
            } else {
                Err(format!("Expected dict, got {:?}", v).into())
            }
        }
        Some(key) => {
            // object
            if v.is_object() {
                hf_summarize_(v, key, _formatters)
            } 
            // array
            else if let Some(arr) = v.as_array() {
                let elements = arr
                    .iter()
                    .map(|v2| reformat_fhir(v2, Some(key), _formatters))
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
                // unlist array if len==1
                match elements.len() {
                    1 => Ok(elements[0].clone()),
                    _ => Ok(serde_json::Value::Array(elements)),
                }
            } 
            // scalar
            else {
                Ok(v.clone())
            }
        }
    }
}

/**
 * Main formatting function. Quite a hack but it works for now.
 */
fn hf_summarize_(_obj: &serde_json::Value, _key: &str, _formatters: &HashMap<String, String>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {

    // special case: Reference
    // Unnest the reference object and wrap into `Reference(...)` for better parseability.
    if let Some(_map) = _obj.as_object() {
        if let Some(_ref) = _map.get("reference") {
            if _map.keys().len() == 1 {
                return Ok(json!(format!("Reference({})", _ref.as_str().unwrap())));
            }
            else {
                let mut _new_map = _map.clone();
                _new_map.insert("reference".to_string(), json!(format!("Reference({})", _ref.as_str().unwrap())));
                return Ok(json!(_new_map));
            }
        }

        // apply custom formatters if any
        let _attr = _map.keys().map(|k| k.to_string()).collect::<Vec<String>>();
        let _sign_str = signature_to_str(_attr);

        match _formatters.get(&_sign_str) {
            Some(format_str) => {
                return Ok(json!(apply_format(_obj, format_str)));
            }
            None => {}
        };

        // go deeper and pass subelements back to recursion function
        let reformatted_obj: serde_json::Map<std::string::String, serde_json::Value> = _map
            .iter()
            .map(|(k, v)| Ok((k.clone(), reformat_fhir(v, Some(k), _formatters)?)))
            .collect::<Result<_, Box<dyn std::error::Error>>>()?;

        return Ok(serde_json::Value::Object(reformatted_obj));
    }

    return Ok(_obj.clone());
}

fn to_yaml(obj: &serde_json::Value) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(obj)
}

pub struct FriendlyBuilder {}
impl FriendlyBuilder {
    pub fn with_file(&self, mapping_file: &Path) -> FriendlyBuilderFromMappingFile {
        FriendlyBuilderFromMappingFile {
            mapping_file: mapping_file.to_path_buf()
        }
    }

    pub fn with_string(&self, mapping_str: &str) -> FriendlyBuilderFromMappingString {
        FriendlyBuilderFromMappingString {
            mapping_str: mapping_str.to_string()
        }
    }

    pub fn run(&self, fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        // load custom mappers from file
        let mapping = load_default_mapping()?;
        let _formatters = process_mapping(&mapping)?;
        return _friendly(fhir_obj, &_formatters);
    }
}

pub struct FriendlyBuilderFromMappingFile {
    mapping_file: PathBuf
}
impl FriendlyBuilderFromMappingFile {
    pub fn run(&self, fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        // load custom mappers from file
        let mapping = load_mapping_from_file(self.mapping_file.as_path())?;
        let _formatters = process_mapping(&mapping)?;
        return _friendly(fhir_obj, &_formatters);
    }
}

pub struct FriendlyBuilderFromMappingString {
    mapping_str: String
}
impl FriendlyBuilderFromMappingString {
    pub fn run(&self, fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        // load custom mappers from file
        let mapping = load_mapping_from_str(self.mapping_str.as_str())?;
        let _formatters = process_mapping(&mapping)?;
        return _friendly(fhir_obj, &_formatters);
    }
}

pub fn friendly() -> FriendlyBuilder {
    FriendlyBuilder{}
}

fn _friendly(fhir_obj: serde_json::Value, formatters: &HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
    // reformat FHIR object
    let reformatted_obj = reformat_fhir(&fhir_obj, None, &formatters)?;
    Ok(to_yaml(&reformatted_obj)?)
}


#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_read_mapping() {
        let file_path = "../resources/mapping.hfc";
        println!("Reading mapping file: {}", file_path);
        let _ = process_mapping(&file_path);
        //println!("{:?}", mapping);
    }

    #[test]
    fn test_xjsonp() {
        let json_string = r#"{
            "use": "usual",
            "type": {
                "coding": [
                    {
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR"
                    }
                ]
            },
            "system": "urn:oid:"
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(v) => {
                match xjsonp(&v, "$..code") {
                    Ok(s) => {
                        assert_eq!(s, r#"["MR"]"#);
                    }
                    Err(e) => {
                        eprintln!("Error parsing JSON: {}", e);
                        assert!(false);
                    }
                }
                assert_eq!(xjsonp_first(&v, "$..code"), "MR");
                assert_eq!(xjsonp_first(&v, "$..notthere"), "");
                assert_eq!(xjsonp_first(&v, "$..type"), "");
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_contains_objects_or_arrays_1() {

        let json_string = r#"{
            "use": "usual",
            "type": {
                "coding": [
                    {
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR"
                    }
                ]
            },
            "system": "urn:oid:"
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(response) => {
                if let Some(_obj) = response.as_object() {
                    assert!(contains_objects_or_arrays(_obj));
                }
                else {
                    assert!(false);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_contains_objects_or_arrays_2() {

        let json_string = r#"{
            "use": "usual",
            "coding": [
                {
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                    "code": "MR"
                }
            ],
            "system": "urn:oid:"
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(response) => {
                if let Some(_obj) = response.as_object() {
                    assert!(contains_objects_or_arrays(_obj));
                }
                else {
                    assert!(false);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_contains_objects_or_arrays_3() {

        let json_string = r#"{
            "use": "usual",            
            "system": "urn:oid:"
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(response) => {
                if let Some(_obj) = response.as_object() {
                    assert!(!contains_objects_or_arrays(_obj));
                }
                else {
                    assert!(false);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_joined_array_1() {

        let json_string = r#"{
            "given": ["Anna-Maria", "Magdalena", "Luisa"]
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(response) => {
                if let Some(_obj) = response.as_object() {
                    assert_eq!(joined_arr(_obj, "given"), Some(json!("Anna-Maria Magdalena Luisa")));
                }
                else {
                    assert!(false);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_joined_array_2() {

        let json_string = r#"{
            "foo": ["Anna-Maria", "Magdalena", "Luisa"]
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(response) => {
                if let Some(_obj) = response.as_object() {
                    assert_eq!(joined_arr(_obj, "given"), None);
                }
                else {
                    assert!(false);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_joined_array_3() {

        let json_string = r#"{
            "given": [
                {"hello": "Anna-Maria"},
                {"hi": "Magdalena"},  
                {"ciao": "Luisa"}
            ]
        }"#;

        match serde_json::from_str::<serde_json::Value>(json_string) {
            Ok(response) => {
                if let Some(_obj) = response.as_object() {
                    assert_eq!(joined_arr(_obj, "given"), Some(json!("")));
                }
                else {
                    assert!(false);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                assert!(false);
            }
        }
    }
}