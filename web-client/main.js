let msgBox = document.getElementById('msg');

navigator.serviceWorker.register("worker.js").then(function(worker) {
  console.log("Registered worker:", worker);
}).catch(function(err) {
  console.log("Failed to register worker:", err);
});

navigator.serviceWorker.ready.then(function(worker) {
  console.log("Worker ready! ", arguments);
  worker.pushManager.subscribe({
    "userVisibleOnly": true
  }).then(function(sub) {
    console.log("Subscription:", sub);
    msgBox.innerText = "Successfully registered, endpoint: " + sub.endpoint;
  }).catch(function(err) {
    console.log("Failed to register push", err);
    msgBox.innerText = "Failed to register, make sure push is allowed and check your logs";
  });
});
