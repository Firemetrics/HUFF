use serde_json::json;

fn main() {
    let fhir_resource = json!({
        "resourceType": "Encounter",
        "id": "example",
        "text": {
          "status": "generated",
          "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">Encounter with patient @example</div>"
        },
        "status": "in-progress",
        "class": {
          "system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
          "code": "IMP",
          "display": "inpatient encounter"
        },
        "subject": {
          "reference": "Patient/example"
        }
    });

    let hff = hff_rs::builder().run(fhir_resource).unwrap();
    println!("{hff}");
}
