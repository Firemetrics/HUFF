# HUFF / Human-friendly FHIR (no pun intended)

## Build
```bash
./buid.sh
```

## Run
```bash
export AUTH_TOKEN="eyJhbGciOi..."
curl -s --header 'Authorization: Bearer '"$AUTH_TOKEN"'' "https://fhir.com/Practitioner/1234" | hff-rs/lib/target/release/hff | yh
```

## Install chrome extension
Just build and load the `chrome-extension` directory as an unpacked extension.