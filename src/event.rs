pub struct Event {
    pub header: String,
    pub body: Option<String>,
}

impl Event {
    pub fn new(header: String, body: Option<String>) -> Self {
        Self { header, body }
    }
}
