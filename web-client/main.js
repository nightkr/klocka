let msgBox = document.getElementById('msg');

let ringtone = new Audio("ringtone.ogg");
let channel = new MessageChannel();

msgBox.innerText = "Trying to register push notifications...";

if (navigator.serviceWorker === undefined) {
  msgBox.innerText = "Sorry, your browser doesn't support service workers. :(";
} else if (Notification === undefined) {
  msgBox.innerText = "Sorry, your browser doesn't support notifications. :(";
}

navigator.serviceWorker.register("worker.js").then(function(worker) {
  console.log("Registered worker:", worker);
}).catch(function(err) {
  console.log("Failed to register worker:", err);
});

Notification.requestPermission().then(function() {
  return navigator.serviceWorker.ready;
}).then(function(worker) {
  console.log("Worker ready! ", arguments);

  channel.port1.onmessage = function(msg) {
    if (msg.data === "open") {
      ringtone.play();
    }
  };
  worker.active.postMessage("channel", [channel.port2]);

  if (worker.pushManager === undefined) {
    msgBox.innerText = "Sorry, your browser doesn't support W3C Web Push. :("
  }
  return worker.pushManager.subscribe({
    "userVisibleOnly": true
  })
}).then(function(sub) {
  console.log("Subscription:", sub);

  if (sub.getKey !== undefined) {
    msgBox.innerText = "Sorry, Klocka doesn't currently support Firefox's Push encryption. :(";
  }

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
