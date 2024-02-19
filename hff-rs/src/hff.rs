use serde_json;
use serde_json::json;
use serde_yaml;

fn reformat_fhir(v: &serde_json::Value, k: Option<&str>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match k {
        None => {
            if let Some(obj) = v.as_object() {
                let reformatted_obj: serde_json::Map<std::string::String, serde_json::Value> = obj
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), reformat_fhir(v, Some(k))?)))
                    .collect::<Result<_, Box<dyn std::error::Error>>>()?;
                Ok(serde_json::Value::Object(reformatted_obj))
            } else {
                Err(format!("Expected dict, got {:?}", v).into())
            }
        }
        Some(key) => {
            if let Some(obj) = v.as_object() {
                hf_summarize(obj, key)
            } 
            else if let Some(arr) = v.as_array() {
                let elements = arr
                    .iter()
                    .map(|v2| reformat_fhir(v2, Some(key)))
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
                match elements.len() {
                    1 => Ok(elements[0].clone()),
                    _ => Ok(serde_json::Value::Array(elements)),
                }
            } 
            else {
                Ok(v.clone())
            }
        }
    }
}

fn hf_telecom(obj: &serde_json::Map<String, serde_json::Value>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    
    let keys = obj.keys().map(|k| k.as_str()).collect::<Vec<&str>>();
    let sliced = keys.as_slice();

    match &sliced[..] {
        &["system", "value", "use"] => {
            if let (Some(serde_json::Value::String(value)), Some(serde_json::Value::String(system)), Some(serde_json::Value::String(use_val))) =
                (obj.get("value"), obj.get("system"), obj.get("use"))
            {
                return Ok(json!(format!("{} | {} | {}", value, system, use_val)));
            }
        }
        &["system", "value"] => {
            if let (Some(serde_json::Value::String(value)), Some(serde_json::Value::String(system))) =
                (obj.get("value"), obj.get("system"))
            {
                return Ok(json!(format!("{} | {}", value, system)));
            }
        }
        _ => {}
    }

    reformat_fhir(&json!(obj), None).map_err(|e| e.into())
}

fn joined_arr(o: &serde_json::Map<String, serde_json::Value>, k: &str) -> Option<serde_json::Value> {
    if let Some(_val) = o.get(k) {
        if let Some(_val_arr) = _val.as_array() {
            return Some(json!(_val_arr.iter().filter_map(|opt| opt.as_str()).collect::<Vec<&str>>().join(" ")));
        }
    }
    return None
}

fn hf_name(obj: &serde_json::Map<String, serde_json::Value>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {

    if let Some(text) = obj.get("text") {
        return Ok(json!(text));
    }
    
    let given = joined_arr(obj, "given");        
    let family = obj.get("family");
    let prefix = obj.get("prefix");
    let suffix = obj.get("suffix");
    
    let _name_vec = vec![prefix, given.as_ref(), family, suffix];
    let _name = _name_vec.iter().filter_map(|opt| opt.as_ref().and_then(|v| v.as_str())).collect::<Vec<&str>>().join(" ");

    if let Some(_use) = obj.get("use") {
        return Ok(json!(format!("{} | {}", _name, _use.as_str().unwrap())));
    } else {
        return Ok(json!(_name));
    }
}

fn hf_summarize(_obj: &serde_json::Map<String, serde_json::Value>, _key: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {

    // special case: Reference
    if let Some(_ref) = _obj.get("reference") {
        if _obj.keys().len() == 1 {
            return Ok(json!(format!("Reference({})", _ref.as_str().unwrap())));
        }
    }
    
    // special case: telecom
    if _key == "telecom" {
        return hf_telecom(_obj);
    }

    // special case: name
    if _key == "name" {
        return hf_name(_obj);
    }

    if !(_obj.iter().any(|(_, v)| v.is_object())) {
        // generic case
        // this is just a quick and dirty way to summarize the object
        // as it omits tags that are not explicitly listed
        let tags = [
            "system", "value", "unit", "code", "version", "display", 
            "url", "valueInstant", "valueString", "valueBoolean", "valueCode"
        ];

        // prioritized tags
        let mut prim_tags = tags.iter()        
            .filter_map(|&tag| match _obj.get(tag) { 
                Some(v) => Some(v.as_str().unwrap().to_string()),
                None => None
            })
            .collect::<Vec<String>>();

        // left-over tags
        let sec_tags = _obj.keys()
            .filter(|&k| !tags.contains(&k.as_str()))
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        // concatenate tags    
        prim_tags.extend(sec_tags);
        let summary = prim_tags.join(" | ");

        if !summary.is_empty() {
            return Ok(json!(summary));
        }
    }

    let reformatted_obj: serde_json::Map<std::string::String, serde_json::Value> = _obj
        .iter()
        .map(|(k, v)| Ok((k.clone(), reformat_fhir(v, Some(k))?)))
        .collect::<Result<_, Box<dyn std::error::Error>>>()?;
    return Ok(serde_json::Value::Object(reformatted_obj))

}

fn to_yaml(obj: &serde_json::Value) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(obj)
}

pub fn friendly(fhir_obj: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let reformatted_obj = reformat_fhir(&fhir_obj, None)?;
    Ok(to_yaml(&reformatted_obj)?)
}
