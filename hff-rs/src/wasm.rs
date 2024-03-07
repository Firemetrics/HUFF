use serde_json;
use serde_json::json;
use wasm_bindgen::prelude::*;

mod hff;

/** 
 * To be called from JavaScript. Input should be a JSON-FHIR string.
 * Result is a JSON string with a "success" boolean and a "yaml" or "error" string.
 */
#[wasm_bindgen]
pub fn js_fhir_to_huff(fhir_str: &str) -> String {
    match serde_json::from_str(fhir_str) {
        Ok(fhir_obj) => 
            match hff::builder().run(fhir_obj) {
                Ok(friendly_yaml) => json!({ "success": true, "yaml": friendly_yaml }).to_string(),
                Err(e) => json!({ "success": false, "error": e.to_string() }).to_string(),
            },
        
        Err(e) => json!({ "success": false, "error": e.to_string() }).to_string(),
    }
}

/** 
 * To be called from JavaScript. Input should be a JSON-FHIR string and an additional string that passes a custom mapping profile into the hff lib.
 * Result is a JSON string with a "success" boolean and a "yaml" or "error" string.
 */
#[wasm_bindgen]
pub fn js_fhir_to_huff_custom(fhir_str: &str, mapping_str: &str) -> String {
    match serde_json::from_str(fhir_str) {
        Ok(fhir_obj) => 
            match hff::builder().with_string(mapping_str).run(fhir_obj) {
                Ok(friendly_yaml) => json!({ "success": true, "yaml": friendly_yaml }).to_string(),
                Err(e) => json!({ "success": false, "error": e.to_string() }).to_string(),
            },
        
        Err(e) => json!({ "success": false, "error": e.to_string() }).to_string(),
    }
}

/** 
 * Pass the default mapping to the caller. This will most likely be used as a starting point for custom mappings.
 */
#[wasm_bindgen]
pub fn default_mapping_js() -> String {
    hff::mapping::default_mapping().to_string()
}