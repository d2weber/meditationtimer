cacheName = 'v4'

self.addEventListener('install', function(event) {
  console.log('[Service Worker] Install');
  event.waitUntil(
    caches.open(cacheName).then(function(cache) {
      return cache.addAll([
        '/', // todo: pkg/..
        '/_api/public/meditationtimer.webmanifest',
        '/_api/public/icon-256.png',
        '/_api/public/icon-616.png',
        '/_api/public/background-leaves.png',
        '/_api/public/silence_1s.wav',
        '/_api/public/tibetan-bowl-low.ogg',
        '/_api/public/tibetan-bowl.ogg'
      ]);
    })
  );
});

self.addEventListener('fetch', (e) => {
  e.respondWith((async () => {
    const r = await caches.match(e.request);
    console.log(`[Service Worker] Fetching resource: ${e.request.url}`);
    if (r) return r;
    const response = await fetch(e.request);
    const cache = await caches.open(cacheName);
    console.log(`[Service Worker] Caching new resource: ${e.request.url}`);
    cache.put(e.request, response.clone());
    return response;
  })());
});

self.addEventListener('activate', (e) => {
  e.waitUntil(caches.keys().then((keyList) => {
    return Promise.all(keyList.map((key) => {
      if (key === cacheName) { return; }
      return caches.delete(key);
    }))
  }));
});
