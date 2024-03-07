
import init, { default_mapping_js } from "./static/hff-wasm.js";

// Restores select box and checkbox state using the preferences
// stored in chrome.storage.
const restoreOptions = async () => {

    console.log('restoring options');

    // load content of src/styles.css into customCssStyles textarea
    let css = '';
    const response = await fetch("styles.css");
    if (response.ok) {
        css = await response.text();
    }

    // init wasm and get default mappings
    await init();
    let mappings = default_mapping_js();

    let contentTypes = [
        'application/fhir+json',
        'application/json+fhir'
    ];

    chrome.storage.sync.get(
        { 
            highlightHuff: true, 
            makeLinksClickable: true, 
            makeReferencesClickable: true, 
            handleContentTypes: contentTypes.join('\n'),
            customCssStyles: css,
            customMappings: mappings
        },
        (items) => {
            document.getElementById('highlightHuff').checked = items.highlightHuff;
            document.getElementById('makeLinksClickable').checked = items.makeLinksClickable;
            document.getElementById('makeReferencesClickable').checked = items.makeReferencesClickable;
            document.getElementById('handleContentTypes').value = items.handleContentTypes;
            document.getElementById('customCssStyles').value = items.customCssStyles;
            document.getElementById('customMappings').value = items.customMappings;
        }
    );
};
  
// Saves options to chrome.storage
const saveOptions = () => {

    const highlightHuff = document.getElementById('highlightHuff').checked;
    const makeLinksClickable = document.getElementById('makeLinksClickable').checked;
    const makeReferencesClickable = document.getElementById('makeReferencesClickable').checked;
    const handleContentTypes = document.getElementById('handleContentTypes').value;
    const customCssStyles = document.getElementById('customCssStyles').value;
    const customMappings = document.getElementById('customMappings').value;

    chrome.storage.sync.set(
        { 
            highlightHuff: highlightHuff, 
            makeLinksClickable: makeLinksClickable, 
            makeReferencesClickable: makeReferencesClickable, 
            handleContentTypes: handleContentTypes, 
            customCssStyles: customCssStyles,
            customMappings: customMappings
        },
        () => {
            // Update status to let user know options were saved.
            const status = document.getElementById('status');
            status.textContent = 'Options saved.';
            setTimeout(() => {
                status.textContent = '';
            }, 750);
        }
    );
};

document.addEventListener('DOMContentLoaded', restoreOptions);
document.getElementById('save').addEventListener('click', saveOptions);