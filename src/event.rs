use std::{collections::HashMap, hash::Hash};

use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct Event {
    pub header: HashMap<String, String>,
    pub body: Option<Value>,
}

impl Event {
    pub fn new(header: String, body: Option<String>) -> Self {
        let header = header
            .split("\n")
            .map(|s| {
                let mut iter = s.split(":");
                let key = iter.next().unwrap().trim().to_lowercase();
                let value = iter.next().unwrap().trim().to_string();
                (key, value)
            })
            .collect::<HashMap<String, String>>();
        let body = body.map(|s| serde_json::from_str(&s).unwrap());
        Self { header, body }
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.header.get(key).map(|s| s.to_string())
    }

    // get job-uuid
    pub fn get_job_uuid(&self) -> Option<String> {
        self.get_header(&"Job-UUID".to_lowercase())
            .map(|s| s.to_string())
    }
}
