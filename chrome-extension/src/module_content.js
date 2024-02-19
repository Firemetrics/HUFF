
import hljs from 'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/es/highlight.min.js';
import yaml from 'https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/es/languages/yaml.min.js';
hljs.registerLanguage('yaml', yaml);

import * as linkify from 'https://cdn.jsdelivr.net/npm/linkifyjs@4.1.3/+esm';
import linkifyHtml from "https://cdn.jsdelivr.net/npm/linkify-html@4.1.3/+esm";

import init, { friendly_js } from "./static/hff-wasm.js";

// inject the highlight.js stylesheet
const hljsStyles = document.createElement('link');
hljsStyles.rel = 'stylesheet';
hljsStyles.href = 'https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.9.0/build/styles/dark.min.css';
document.head.appendChild(hljsStyles);


init().then(() => {

    // inject preferenes into the page
    const hidden = document.getElementById('extPrefs');
    const extPrefs = JSON.parse(hidden.value);
    
    // get rid of the source code checkbox
    document.body.removeChild(document.querySelector('body > div'));

    // format all pre elements (should be just one)
    const preElements = document.getElementsByTagName('pre');
    for (const preElement of preElements) {
        try {            
            
            // Create HUFF
            const result = JSON.parse(friendly_js(preElement.textContent));

            if (result.success === true) {
                let yaml = result.yaml;
                // highlight yaml
                if (extPrefs.highlightHuff) {
                    yaml = hljs.highlight(yaml, {language: 'yaml'}).value;
                }

                // posprocess to make relatve URLs absolute
                //yaml = yaml.replace(/\>((?:\/|\.\.?\/)[^\s]+(?:\/|\b))/g, '>'+window.location.origin+'$1');

                // postprocess to make all URLs clickable
                if (extPrefs.makeLinksClickable) {
                    yaml = linkifyHtml(yaml, {});
                }
                
                // postprocess Reference() to make it clickable
                if (extPrefs.makeReferencesClickable) {
                    // poor algo to get the FHIR server base URL
                    let fhirServerUrl = window.location.origin;
                    if (window.location.search.length > 0) {
                        fhirServerUrl += window.location.pathname.split('/').slice(0, -1).join('/');
                    }
                    else {
                        fhirServerUrl += window.location.pathname.split('/').slice(0, -2).join('/');
                    }

                    yaml = yaml.replace(/Reference\((.+?)\)/g, '<a href="'+fhirServerUrl+'/$1">$1</a>');
                }

                // update document
                preElement.innerHTML = `<code class="language-yaml">${yaml}</code>`;
            }
            else {
                console.log("Error in WASM code: " + result.error);
            }
        } catch (error) {
            console.error("Unexpected error: " + error);
            continue;
        }
    }
});