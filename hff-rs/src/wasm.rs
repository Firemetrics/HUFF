use serde_json;
use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::hff::friendly;
mod hff;

/** 
 * To be called from JavaScript. Input should be a JSON-FHIR string.
 * Result is a JSON string with a "success" boolean and a "yaml" or "error" string.
 */
#[wasm_bindgen]
pub fn friendly_js(fhir_str: &str) -> String {
    match serde_json::from_str(fhir_str) {
        Ok(fhir_obj) => {
            match friendly(fhir_obj) {
                Ok(friendly_yaml) => {
                    return json!({ "success": true, "yaml": friendly_yaml }).to_string()
                }
                Err(e) => {
                    return json!({ "success": false, "error": e.to_string() }).to_string()
                }
            }
        }
        Err(e) => {
            return json!({ "success": false, "error": e.to_string() }).to_string()
        }
    }
}