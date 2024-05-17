use serde_json;
use serde_json::json;
use serde_yaml;
use std::collections::HashMap;

use crate::hff::mapping;

pub fn json_to_huff(
    fhir_obj: serde_json::Value,
    formatters: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // reformat FHIR object
    let reformatted_obj = traverse_fhir(&fhir_obj, None, &formatters)?;
    Ok(json_to_yaml(&reformatted_obj)?)
}

fn json_to_yaml(obj: &serde_json::Value) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(obj)
}

/**
 * Recurse over JSON tree and pass branches to reformatting function.
 */
fn traverse_fhir(
    v: &serde_json::Value,
    k: Option<&str>,
    _formatters: &HashMap<String, String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match k {
        // First-pass of JSON structure
        None => {
            if let Some(obj) = v.as_object() {
                let reformatted_obj: serde_json::Map<std::string::String, serde_json::Value> = obj
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), traverse_fhir(v, Some(k), _formatters)?)))
                    .collect::<Result<_, Box<dyn std::error::Error>>>()?;
                Ok(serde_json::Value::Object(reformatted_obj))
            } else {
                Err(format!("Expected dict, got {:?}", v).into())
            }
        }
        Some(key) => {
            // object
            if v.is_object() {
                reformat(v, key, _formatters)
            }
            // array
            else if let Some(arr) = v.as_array() {
                let elements = arr
                    .iter()
                    .map(|v2| traverse_fhir(v2, Some(key), _formatters))
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
fn reformat(
    _obj: &serde_json::Value,
    _key: &str,
    _formatters: &HashMap<String, String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // special case: Reference
    // Unnest the reference object and wrap into `Reference(...)` for better parseability.
    if let Some(_map) = _obj.as_object() {
        if let Some(_ref) = _map.get("reference") {
            // if there is only the 'reference' key, we can just replace the object with the new value
            if _map.keys().len() == 1 {
                return Ok(json!(format!("Reference({})", _ref.as_str().unwrap())));
            }
            // in some cases, the 'reference' key is nested in a key named 'reference' :P
            // https://hl7.org/fhir/R4/consent-definitions.html#Consent.provision.actor.reference
            else if let Some(reference) = _ref.as_object().and_then(|obj| obj.get("reference")) {
                let mut _new_map = _map.clone();
                _new_map.insert(
                    "reference".to_string(),
                    json!(format!("Reference({})", reference.as_str().unwrap())),
                );
                return Ok(json!(_new_map));
            }
            // if there are other keys, we need to keep them and just modify the 'reference' key
            else {
                let mut _new_map = _map.clone();
                _new_map.insert(
                    "reference".to_string(),
                    json!(format!("Reference({})", _ref.as_str().unwrap())),
                );
                return Ok(json!(_new_map));
            }
        }

        // apply custom formatters if any
        let _attr = _map.keys().map(|k| k.to_string()).collect::<Vec<String>>();
        let _sign_str = mapping::signature_to_str(_attr);

        match _formatters.get(&_sign_str) {
            Some(format_str) => {
                return Ok(json!(mapping::apply_format(_obj, format_str)));
            }
            None => {}
        };

        // go deeper and pass subelements back to recursion function
        let reformatted_obj: serde_json::Map<std::string::String, serde_json::Value> = _map
            .iter()
            .map(|(k, v)| Ok((k.clone(), traverse_fhir(v, Some(k), _formatters)?)))
            .collect::<Result<_, Box<dyn std::error::Error>>>()?;

        return Ok(serde_json::Value::Object(reformatted_obj));
    }

    return Ok(_obj.clone());
}
