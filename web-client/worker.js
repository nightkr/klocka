self.addEventListener('install', function(ev) {
  self.skipWaiting();
  console.log("Installed", ev);
});

self.addEventListener('push', function(ev) {
  console.log(ev);
  registration.showNotification("Open the door!");
})
