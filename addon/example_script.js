/*
On a click on the browser action, send the app a message.
*/
browser.browserAction.onClicked.addListener(() => {
  console.log("Sending:  ping");
  var sending = browser.runtime.sendNativeMessage(
    "example_name",
    "ping");
  sending.then(onResponse, onError);
});

function onResponse(response) {
  console.log("Received " + response["msg"]);
}

function onError(error) {
  console.log(`Error: ${error}`);
}
