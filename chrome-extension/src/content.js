// global preferences
var extPrefs = {};

const getCustomStyles = async () => {
    // load content of src/styles.css into customCssStyles textarea
    let css = '';
    const response = await fetch(chrome.runtime.getURL("src/styles.css"));
    if (response.ok) {
        css = await response.text();
    }
    return css;
};

// Restores select box and checkbox state using the preferences
// stored in chrome.storage.
const restoreOptions = async () => {

    // load content of src/styles.css into customCssStyles textarea
    let css = await getCustomStyles();

    let contentTypes = [
        'application/fhir+json',
        'application/json+fhir'
    ].join('\n');

    extPrefs = await chrome.storage.sync.get(
        { 
            highlightHuff: true, 
            makeLinksClickable: true, 
            makeReferencesClickable: true, 
            handleContentTypes: contentTypes,
            customCssStyles: css,
            customMappings: ''
        }
    );

    console.log(extPrefs);
};

/**
 * The background script will inform us if the current page is FHIR-JSON.
 * If it is, we will inject the HUFF module into the page.
 */
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {    
    if (message.isJsonFHIR) {
        inject();
    }
});

const inject = async () => {

    await restoreOptions();

    // inject preferenes into the page
    const hidden = document.createElement('input');
    hidden.type = 'hidden';
    hidden.id = 'extPrefs';
    hidden.value = JSON.stringify(extPrefs);
    document.body.appendChild(hidden);

    // ugly hack but content.js is not able to import modules
    // so we need to inject another module script into the page
    const scriptElement = document.createElement('script');
    scriptElement.type = 'module';
    scriptElement.src = chrome.runtime.getURL("src/module_content.js");
    document.body.appendChild(scriptElement);

    if (extPrefs.highlightHuff) {
        if (extPrefs.customCssStyles.length > 0) {
            // inject our custom styles
            const extStyles = document.createElement('style');
            extStyles.innerHTML = extPrefs.customCssStyles;
            document.head.appendChild(extStyles);
        }
        else {
            // inject the highlight.js stylesheet
            const hljsStyles = document.createElement('link');
            hljsStyles.rel = 'stylesheet';
            hljsStyles.href = 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.9.0/build/styles/dark.min.css';
            document.head.appendChild(hljsStyles);
        }
    }
};