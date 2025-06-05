![Banner](./screenshots/HUFF-banner-new.png)

# HUFF / Human-friendly FHIR (no pun intended)

[![Affiliated with RTG WisPerMed](https://img.shields.io/badge/Affiliated-RTG%202535%20WisPerMed-blue)](https://wispermed.org/)

## WTF is HUFF? And why?
The dense and nested structure of JSON, filled with parentheses and commas, can be overwhelming, making it difficult to quickly grasp the hierarchy and data relationships in FHIR resources. In addition, a typical FHIR resource is a complex JSON object with many nested objects and arrays. This makes it difficult to read and understand the data. Also, FHIR comes with a lot of repeating patterns and structures, e.g., `system`, `value`, or `system`, `value`, `display`, or `value`, `unit`, `system`. You know what I mean. For experienced users, these can easily be joined into one line without losing much expressiveness.



So what is HUFF? It is simply a condensation of the JSON tree into a more human readable structure. It's like turning a big thorny bush into a cute little bonsai tree - easier on the eyes and less prickly. And well... then the whole thing is transformed into YAML, because its cleaner, indentation-based format offers a more intuitive and visually organized alternative. 

It's not a new format, it's just a different way of presenting the same data. It's not a replacement for JSON, it's just a different way of looking at the same data. 

Just be friendly.

## Build
```bash
./build.sh
```

## Run

Load FHIR from FHIR server via `curl` (expects the server to answer with JSON by default, otherwise specify explicitly). Pass authentication token to `curl` via ENV variable. Pipe `curl` output to `hff` and then to `yh` for syntax highlighting (install `yh` via `apt`, `brew`, ..).
```bash
export AUTH_TOKEN="eyJhbGciOi..."
curl -s --header 'Authorization: Bearer '"$AUTH_TOKEN"'' "https://fhir.com/Practitioner/1234" | hff | yh
```

Run with custom mappings.
```bash
export AUTH_TOKEN="eyJhbGciOi..."
curl -s --header 'Authorization: Bearer '"$AUTH_TOKEN"'' "https://fhir.com/Practitioner/1234" | hff -m "./my/custom/mappings.hfc" | yh
```

## Install chrome extension
Just build and load the `chrome-extension` directory as an unpacked extension. You can modify the mappings (in *.hfc format) in the extension options.

### Chrome extension screenshots

URL: https://demo.kodjin.com/fhir/Patient/f8ac8f25-99fd-471e-befa-5a0b49bbe9a3

JSON presentation of the FHIR resource:
![JSON](./screenshots/kodjin-patient-json.png)

HUFF presentation of the same FHIR resource:
![HUFF](./screenshots/kodjin-patient-huff.png)

## Customization at build time
You can customize the output by editing the `hff-rs/resources/mapping.hfc` file. The syntax is a simple format. The `hff-rs/resources/mapping.hfc` file is a good starting point. Quickly, the format is:
- Lines starting with `//` are comments
- Lines starting with `#` specify the signature a JSON node needs to match to be processed by the following line. E.g. if the signature is `#[value, unit, system]` think of it as beeing equal to the JSONPath `$.*[?(@.hasOwnProperty('value') && @.hasOwnProperty('unit') && @.hasOwnProperty('system'))]`. The reason why this is not implemented as a JSONPath in the first place is just KIS but this might change in future.
- The line after the signature specifies the reformatting of the JSON node. You can specifiy any string here where the content of the `{..}` placeholder is interpreted as a JSONPath expression relative to the current node. E.g. `{$.family}` will extract the value of the `family` property of the current node. If the JSONPath expression returns an array this is automatically joined via whitespace into a string. 
