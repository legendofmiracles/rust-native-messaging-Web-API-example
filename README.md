# rust-native-messaging-Web-API-example
This is a example repository, for people to use, to quickly set up native messaging with rust from browsers.

## Setup
clone this repo, and replace all the values, in the addon directory, and in the example_name.json file.

Then copy the example_name.json (or however you know call it, into one of the directories/regedit as explained on this this: [link](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests) (You'll have to scoll down)

Make sure all of the values are correct, and you should be good to go.

Run the extension as explained here: https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_first_WebExtension, under the installing page or use web-ext (install with npm: `npm install -g web-ext`).

For chrome, follow these steps:
1) go to chrome://extensions
2) Load unpacked
3) Choose folder
and probably have to turn on developer mode before you can load unpacked in the top right in chrome://extensions
