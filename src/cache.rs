use dashmap::DashMap;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::time::interval;

use std::{
    iter,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::constants::EXTERNAL_URL;

fn random_identifier() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(20)
        .collect()
}

#[derive(Clone)]
pub struct ImageCache(Arc<DashMap<String, (Vec<u8>, Instant)>>);

impl ImageCache {
    pub fn new() -> Self {
        let map = Arc::new(DashMap::new());
        let cache = Self(map);
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

    pub fn get(&self, identifier: &str) -> Option<Vec<u8>> {
        self.0.get(identifier).map(|image| image.0.clone())
    }

    pub fn insert(&self, image: Vec<u8>) -> String {
        let identifier = random_identifier();
        self.0.insert(identifier.clone(), (image, Instant::now()));

        format!("{}/image?image={}", *EXTERNAL_URL, identifier)
    }
}
