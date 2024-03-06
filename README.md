# HUFF / Human-friendly FHIR (no pun intended)

## Build
```bash
./build.sh
```

## Run
```bash
export AUTH_TOKEN="eyJhbGciOi..."
curl -s --header 'Authorization: Bearer '"$AUTH_TOKEN"'' "https://fhir.com/Practitioner/1234" | hff-rs/lib/target/release/hff | yh
```

## Install chrome extension
Just build and load the `chrome-extension` directory as an unpacked extension.

## Customization
You can customize the output by editing the `hff-rs/resources/mapping.hfc` file. The syntax is a simple format. The `hff-rs/resources/mapping.hfc` file is a good starting point. Quickly, the format is:
- Lines starting with `//` are comments
- Lines starting with `#` specify the signature a JSON node needs to match to be processed by the following line. E.g. if the signature is `#[value, unit, system]` think of it as beeing equal to the JSONPath `$.*[?(@.hasOwnProperty('value') && @.hasOwnProperty('unit') && @.hasOwnProperty('system'))]`. The reason why this is not implemented as a JSONPath in the first place is just KIS but this might change in future.
- The line after the signature specifies the reformatting of the JSON node. You can specifiy any string here where the content of the `{..}` placeholder is interpreted as a JSONPath expression relative to the current node. E.g. `{$.family}` will extract the value of the `family` property of the current node. If the JSONPath expression returns an array this is automatically joined via whitespace into a string. 