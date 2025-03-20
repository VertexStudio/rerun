var cacheName = "rerun-viewer-pwa-v1";
var filesToCache = [
  "./",
  "./index.html",
  "./re_viewer.js",
  "./re_viewer_bg.wasm",
];

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  self.skipWaiting();

  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    }),
  );
});

/* When a new service worker takes over, clear old caches */
self.addEventListener("activate", function(e) {
  e.waitUntil(
    caches.keys().then(function(keyList) {
      return Promise.all(keyList.map(function(key) {
        if (key !== cacheName) {
          return caches.delete(key);
        }
      }));
    })
  );
});

/* Serve cached content when offline */
self.addEventListener("fetch", function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      if (e.request.url.endsWith(".wasm")) {
        return fetch(e.request).catch(function() {
          return response;
        });
      }
      return response || fetch(e.request);
    }),
  );
});
