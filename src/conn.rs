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
