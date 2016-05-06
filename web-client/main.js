navigator.serviceWorker.register("worker.js").then(function(worker) {
  console.log("Registered worker:", worker);

  worker.pushManager.subscribe({
    "userVisibleOnly": true
  }).then(function(sub) {
    console.log("Subscription:", sub);
  }).catch(function(err) {
    console.log("Failed to register push", err);
  });
}).catch(function(err) {
  console.log("Failed to register worker:", err);
})
