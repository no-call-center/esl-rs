use crate::error::{EslError, Result};
use crate::event::Event;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
#[derive(Debug, Clone)]
pub struct Conn {
    pub(crate) connected: bool,
    pub(crate) sender: Arc<Mutex<Sender<String>>>, // send command
    pub(crate) receiver: Arc<Mutex<Receiver<Event>>>, // receive freesiwtch event
}

impl Conn {
    pub(crate) fn new(
        sender: Arc<Mutex<Sender<String>>>,
        receiver: Arc<Mutex<Receiver<Event>>>,
    ) -> Self {
        Self {
            connected: true,
            sender,
            receiver,
        }
    }

    fn get_header_end(s: &[u8]) -> Option<usize> {
        let mut i = 0;
        let mut last = 0;
        for c in s {
            if *c == b'\n' {
                if last == b'\n' {
                    return Some(i);
                }
                last = *c;
            } else {
                last = *c;
            }
            i += 1;
        }
        None
    }

    fn parse_header(header: &[u8]) -> HashMap<String, String> {
        /*
        Content-Length: 603
        Content-Type: text/event-json
         */
        let mut map = HashMap::new();
        let mut key = String::new();
        let mut value = String::new();
        let mut is_key = true;
        for c in header {
            if *c == b':' {
                is_key = false;
            } else if *c == b'\n' {
                map.insert(key.to_lowercase(), value);
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

    pub async fn send(&self, command: &str) -> Result<()> {
        if !self.connected {
            return Err(crate::error::EslError::ConnectionError(
                "not connected".to_string(),
            ));
        }
        let sender = self.sender.clone();
        let command = format!("{}\n\n", command);
        sender.lock().await.send(command).await.unwrap();
        Ok(())
    }

    // receive event from freeswitch
    pub async fn recv(&mut self) -> Result<Event> {
        if !self.connected {
            return Err(crate::error::EslError::ConnectionError(
                "not connected".to_string(),
            ));
        }
        let event = self
            .receiver
            .lock()
            .await
            .recv()
            .await
            .ok_or(EslError::ApiError("receive event error".to_string()))?;

        Ok(event)
    }

    pub async fn bgapi(&self, command: &str) -> Result<String> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let sender = self.sender.clone();
        let command = format!("bgapi {}\njob-uuid:{}\n\n", command, uuid);
        sender.lock().await.send(command).await.unwrap();
        return Ok(uuid);
    }
}
