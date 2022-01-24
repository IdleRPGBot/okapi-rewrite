use dashmap::DashMap;
use tokio::time::interval;

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::constants::EXTERNAL_URL;

#[derive(Clone)]
pub struct ImageCache(Arc<DashMap<String, (Vec<u8>, Instant)>>, Arc<AtomicU64>);

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageCache {
    #[must_use]
    pub fn new() -> Self {
        let map = Arc::new(DashMap::new());
        let counter = Arc::new(AtomicU64::new(1));
        let cache = Self(map, counter);
        let cache_clone = cache.clone();

        tokio::spawn(async move {
            cache_clone.cleanup_task().await;
        });

        cache
    }

    async fn cleanup_task(&self) {
        // Reap every 5mins
        let mut interval = interval(Duration::from_secs(60 * 5));
        // Keep each for 15mins
        let time_until_reap = Duration::from_secs(60 * 15);

        loop {
            interval.tick().await;

            let right_now = Instant::now();

            self.0
                .retain(|_, (_, created)| *created + time_until_reap > right_now);
        }
    }

    #[must_use]
    pub fn get(&self, identifier: &str) -> Option<Vec<u8>> {
        self.0.get(identifier).map(|image| image.0.clone())
    }

    #[must_use]
    pub fn insert(&self, image: Vec<u8>) -> String {
        let counter = self.1.fetch_add(1, Ordering::SeqCst);
        self.0.insert(counter.to_string(), (image, Instant::now()));

        format!("{}/image?image={}", *EXTERNAL_URL, counter)
    }
}
