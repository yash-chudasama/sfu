use crate::rtc::endpoint::Endpoint;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Room {
    room_id: usize,
    endpoints: Mutex<HashMap<usize, Arc<Endpoint>>>,
}

impl Room {
    pub fn new(room_id: usize) -> Self {
        Self {
            room_id,
            endpoints: Mutex::new(HashMap::new()),
        }
    }

    pub fn room_id(&self) -> usize {
        self.room_id
    }

    pub async fn insert(&self, endpoint: Arc<Endpoint>) {
        let mut endpoints = self.endpoints.lock().await;
        endpoints.insert(endpoint.endpoint_id(), endpoint);
    }

    pub async fn get(&self, endpoint_id: usize) -> Option<Arc<Endpoint>> {
        let endpoints = self.endpoints.lock().await;
        endpoints.get(&endpoint_id).cloned()
    }

    pub async fn remove(&self, endpoint_id: usize) -> Option<Arc<Endpoint>> {
        let mut endpoints = self.endpoints.lock().await;
        endpoints.remove(&endpoint_id)
    }
}
