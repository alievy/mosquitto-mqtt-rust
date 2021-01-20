#[derive(Debug)]
pub struct Message {
    topic: String,
    payload: String,
}

impl Message {
    pub fn new(topic: &str, payload: &str) -> Self {
        Message {
            topic: topic.to_string(),
            payload: payload.to_string(),
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }
}
