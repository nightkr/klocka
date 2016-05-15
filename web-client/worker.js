//let ringtone = new Audio("ringtone.ogg");
let windows = [];

self.addEventListener('install', function(ev) {
  self.skipWaiting();
  console.log("Installed", ev);
});

self.addEventListener('message', function(msg) {
  if (msg.data === "channel") {
    for (let i = 0; i < msg.ports.length; i++) {
      windows.push(msg.ports[i]);
    }
  }
})

self.addEventListener('push', function(ev) {
  console.log(ev);
  for (let i = 0; i < windows.length; i++) {
    windows[i].postMessage("open");
  }
  self.clients.matchAll({"includeUncontrolled": true, "type": "window"}).then(function(clients) {
    if (clients.length > 0) {
      clients[0].postMessage("open");
    }
    console.log(clients);
  });
  //ringtone.play();
  registration.showNotification("Klocka", {
    "body": "Open the door!",
    "sound": "/ringtone.ogg",
    "silent": false,
    "requireInteraction": true
  });
})
