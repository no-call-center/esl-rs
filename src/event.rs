use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn get_header_end(s: &[u8]) -> Option<usize> {
    let mut i = 0;
    let mut last = 0;
    for c in s {
        if *c == b'\n' {
            if last == b'\n' {
                return Some(i + 1);
            }
            last = *c;
        } else {
            last = *c;
        }
        i += 1;
    }
    None
}

pub(crate) fn parse_header(header: &[u8]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut is_key = true;
    for c in header {
        if *c == b':' {
            is_key = false;
        } else if *c == b'\n' {
            map.insert(key, value);
            key = String::new();
            value = String::new();
            is_key = true;
        } else if is_key {
            key.push(*c as char);
        } else {
            value.push(*c as char);
        }
    }
    map
}

#[derive(Debug, Clone, Default)]
pub struct Event {
    pub headers: HashMap<String, String>,
    pub raw_body: Option<String>,
    pub body: Option<HashMap<String, String>>,
}

impl Event {
    pub fn new(headers: HashMap<String, String>, raw_body: Option<String>) -> Self {
        // 如果有body，则json反序列化解析body
        let body = if let Some(body) = raw_body.clone() {
            if let Ok(body) = serde_json::from_str::<Value>(&body) {
                let mut map = HashMap::new();
                if let Value::Object(body) = body {
                    for (k, v) in body {
                        if let Value::String(v) = v {
                            map.insert(k, v);
                        }
                    }
                }
                Some(map)
            } else {
                None
            }
        } else {
            None
        };
        Self {
            headers,
            body,
            raw_body,
        }
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).map(|s| s.to_string())
    }

    pub fn get_job_uuid(&self) -> Option<String> {
        self.get_header(&"Job-UUID").map(|s| s.to_string())
    }

    pub fn get_body(&self) -> Option<&HashMap<String, String>> {
        self.body.as_ref()
    }

    pub fn get_body_by_key(&self, key: &str) -> Option<String> {
        self.get_body()
            .and_then(|v| v.get(key))
            .map(|s| s.to_string())
    }

    pub fn get_event_name(&self) -> Option<String> {
        self.get_body_by_key("Event-Name")
    }

    pub fn get_channel_call_uuid(&self) -> Option<String> {
        self.get_body_by_key("Channel-Call-UUID")
    }
}
