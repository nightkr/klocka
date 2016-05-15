let msgBox = document.getElementById('msg');

navigator.serviceWorker.register("worker.js").then(function(worker) {
  console.log("Registered worker:", worker);
}).catch(function(err) {
  console.log("Failed to register worker:", err);
});

Notification.requestPermission().then(function() {
  return navigator.serviceWorker.ready;
}).then(function(worker) {
  console.log("Worker ready! ", arguments);
  return worker.pushManager.subscribe({
    "userVisibleOnly": true
  })
}).then(function(sub) {
  console.log("Subscription:", sub);
  msgBox.innerText = "Successfully registered, endpoint: " + sub.endpoint;
  let xhr = new XMLHttpRequest();
  xhr.onreadystatechange = function(res) {
    if (xhr.readyState === XMLHttpRequest.DONE) {
      console.log("Submitted endpoint: ", res, xhr);
      if (xhr.status === 200) {
        msgBox.innerText = "Successfully submitted endpoint";
      } else {
        msgBox.innerText = "Failed to submit endpoint";
      }
    }
  };
  xhr.open('POST', '/submit', true);
  xhr.send(sub.endpoint);
}).catch(function(err) {
  console.log("Failed to register push", err);
  msgBox.innerText = "Failed to register, make sure push is allowed and check your logs";
});
