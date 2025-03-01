use serde_json;
use std::path::{Path, PathBuf};

pub mod mapping;
mod reformatting;

pub struct HuffBuilder {}
#[allow(dead_code)]
impl HuffBuilder {
    pub fn with_file(&self, mapping_file: &Path) -> HuffBuilderFromMappingFile {
        HuffBuilderFromMappingFile {
            mapping_file: mapping_file.to_path_buf(),
        }
    }
    pub fn with_string(&self, mapping_str: &str) -> HuffBuilderFromMappingString {
        HuffBuilderFromMappingString {
            mapping_str: mapping_str.to_string(),
        }
    }
    pub fn run(&self, fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        // load custom mappers from file
        let mapping = mapping::load_default_mapping()?;
        let _formatters = mapping::process_mapping(&mapping)?;
        return reformatting::json_to_huff(fhir_obj, &_formatters);
    }
}

#[allow(dead_code)]
pub struct HuffBuilderFromMappingFile {
    mapping_file: PathBuf,
}
#[allow(dead_code)]
impl HuffBuilderFromMappingFile {
    pub fn run(&self, fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        // load custom mappers from file
        let mapping = mapping::load_mapping_from_file(self.mapping_file.as_path())?;
        let _formatters = mapping::process_mapping(&mapping)?;
        return reformatting::json_to_huff(fhir_obj, &_formatters);
    }
}

#[allow(dead_code)]
pub struct HuffBuilderFromMappingString {
    mapping_str: String,
}
#[allow(dead_code)]
impl HuffBuilderFromMappingString {
    pub fn run(&self, fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        // load custom mappers from file
        let mapping = mapping::load_mapping_from_str(self.mapping_str.as_str())?;
        let _formatters = mapping::process_mapping(&mapping)?;
        return reformatting::json_to_huff(fhir_obj, &_formatters);
    }
}

pub fn builder() -> HuffBuilder {
    HuffBuilder {}
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
                } else {
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
                } else {
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
                } else {
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
                    assert_eq!(
                        joined_arr(_obj, "given"),
                        Some(json!("Anna-Maria Magdalena Luisa"))
                    );
                } else {
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
                } else {
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
                } else {
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
