const CACHE_NAME = 'ambidex-v2'; // Increment version to force update
const ASSETS = [
    './',
    './index.html',
    './manifest.json',
    './assets/icon-192.png',
    './assets/icon-512.png',
    // UI Icons
    './assets/ui/icons/gun_rapid.png',
    './assets/ui/icons/gun_shotgun.png',
    './assets/ui/icons/gun_single.png',
    './assets/ui/icons/magic_blink.png',
    './assets/ui/icons/magic_bolt.png',
    './assets/ui/icons/magic_global.png',
    './assets/ui/icons/magic_laser.png',
    './assets/ui/icons/magic_nova.png',
    './assets/ui/icons/shuriken.png',
    './assets/ui/icons/sword_normal.png',
    './assets/ui/icons/sword_shattered.png',
    // Add WASM/JS if needed, usually managed by browser cache or specific caching strategies
    // But for offline, we might need them. However, trunk names change (index-<hash>.js).
    // The ./ alias usually handles index.html.
];

self.addEventListener('install', (event) => {
    self.skipWaiting(); // Force new SW to take control immediately
    event.waitUntil(
        caches.open(CACHE_NAME).then((cache) => {
            console.log('SW: Caching assets');
            return cache.addAll(ASSETS);
        })
    );
});

self.addEventListener('activate', (event) => {
    event.waitUntil(
        Promise.all([
            self.clients.claim(), // Take control of all clients immediately
            caches.keys().then((cacheNames) => {
                return Promise.all(
                    cacheNames.map((cacheName) => {
                        if (cacheName !== CACHE_NAME) {
                            console.log('SW: Deleting old cache', cacheName);
                            return caches.delete(cacheName);
                        }
                    })
                );
            })
        ])
    );
});

self.addEventListener('fetch', (event) => {
    event.respondWith(
        caches.match(event.request).then((response) => {
            // Cache hit - return response
            if (response) {
                return response;
            }
            // Clone request for fetch
            const fetchRequest = event.request.clone();
            return fetch(fetchRequest).then((response) => {
                // Check if valid response
                if (!response || response.status !== 200 || response.type !== 'basic') {
                    return response;
                }
                // Cache new assets dynamically
                const responseToCache = response.clone();
                caches.open(CACHE_NAME).then((cache) => {
                    // Only cache valid schemes (http/https), matching our scope
                    if (event.request.url.startsWith('http')) {
                        cache.put(event.request, responseToCache);
                    }
                });
                return response;
            });
        })
    );
});
