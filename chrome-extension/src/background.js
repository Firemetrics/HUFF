let extPrefs = {};

// Restores select box and checkbox state using the preferences
// stored in chrome.storage.
const restoreOptions = async () => {
    let contentTypes = [
        'application/fhir+json',
        'application/json+fhir'
    ].join('\n');

    extPrefs = await chrome.storage.sync.get({ handleContentTypes: contentTypes });
};

const getCurrentTab = async () => {
    let queryOptions = { active: true, lastFocusedWindow: true };
    let [tab] = await chrome.tabs.query(queryOptions);
    return tab;
}

chrome.webRequest.onHeadersReceived.addListener(async (details) => {

        await restoreOptions();
        const handleContentTypes = extPrefs.handleContentTypes.split('\n');

        // get the content-type header
        const ct = details.responseHeaders.find(header => header.name.toLowerCase() === "content-type");

        // wait for the page to fully load
        let tab = await getCurrentTab();
        while (tab.status !== "complete") {
            tab = await getCurrentTab();
            // wait 10 ms
            await new Promise(resolve => setTimeout(resolve, 10));
        }

        if (ct) {               
            // check if ct constains any string in handleContentTypes
            if (handleContentTypes.some(hct => ct.value.includes(hct))) {                
                try {
                    chrome.tabs.sendMessage(tab.id, {isJsonFHIR: true, contentType: ct.value});
                }
                catch (error) {
                    console.error(error);
                }
            }                  
        }
    },
    {urls: ["<all_urls>"], types: ["main_frame"]},
    ["responseHeaders"]
);
