use crate::error::{EslError, Result};
use crate::event::Event;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
#[derive(Debug, Clone)]
pub struct Conn {
    pub(crate) sender: Arc<Mutex<Sender<String>>>, // send command
    pub receiver: Arc<Mutex<Receiver<Result<Event>>>>, // receive freesiwtch event
    pub connected: Arc<Mutex<bool>>,
}

#[macro_export]
macro_rules! run {
    ($conn:ident) => {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            if let Err(e) = $conn.lock().await.is_connected().await {
                break e;
            }
        }
    };
}

impl Conn {
    pub(crate) fn new(
        sender: Arc<Mutex<Sender<String>>>,
        receiver: Arc<Mutex<Receiver<Result<Event>>>>,
    ) -> Self {
        Self {
            sender,
            receiver,
            connected: Arc::new(Mutex::new(true)),
        }
    }

    pub async fn is_connected(&self) -> Result<bool> {
        let connected = self.connected.clone();
        let connected = connected.lock().await;
        if *connected {
            return Ok(true);
        }

        Err(EslError::ConnectionError)
    }

    pub async fn send(&mut self, command: &str) -> Result<()> {
        self.is_connected().await?;
        let sender = self.sender.clone();
        let sender = sender.lock().await;
        let command = format!("{}\n\n", command);
        match sender.send(command).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("send command error: {}", e);
            }
        };
        Ok(())
    }

    pub async fn handle(&mut self, hander: impl Fn(Event) + Send + Sync + 'static) {
        let receiver = self.receiver.clone();
        let connected = self.connected.clone();

        tokio::spawn(async move {
            let mut receiver = receiver.lock().await;
            while let Some(res) = receiver.recv().await {
                if let Ok(evt) = res {
                    hander(evt);
                } else if let Err(e) = res {
                    log::error!("recv error: {}", e);
                    *connected.lock().await = false;
                }
            }
        });
    }
    pub async fn bgapi(&mut self, command: &str) -> Result<String> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let command = format!("bgapi {}\njob-uuid:{}", command, uuid);
        self.send(&command).await?;
        return Ok(uuid);
    }
}
