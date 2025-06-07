use std::collections::HashSet;
use std::sync::Mutex;

pub struct VisitedUrls {
    urls: Mutex<HashSet<String>>,
}

impl VisitedUrls {
    pub fn new() -> Self {
        Self {
            urls: Mutex::new(HashSet::new()),
        }
    }

    pub fn mark_visited(&self, url: &str) -> bool {
        let mut urls = self.urls.lock().unwrap();
        if urls.contains(url) {
            false
        } else {
            urls.insert(url.to_string());
            true
        }
    }
}
