use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use log::{info, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrawledData {
    pub url: String,
    pub content: String,
    pub timestamp: String,
}

pub struct MockDatabase {
    data: Mutex<HashMap<String, CrawledData>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, url: String, content: String) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let data = CrawledData {
            url,
            content,
            timestamp,
        };
        
        let mut map = self.data.lock().unwrap();
        map.insert(data.url.clone(), data);
        info!("Data inserted into mock database");
        Ok(())
    }

    pub fn get_all(&self) -> Vec<CrawledData> {
        let map = self.data.lock().unwrap();
        map.values().cloned().collect()
    }

    pub fn clear(&self) {
        let mut map = self.data.lock().unwrap();
        map.clear();
        info!("Mock database cleared");
    }
}

pub fn create_mock_database() -> MockDatabase {
    MockDatabase::new()
}