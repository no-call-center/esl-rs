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
    pub(crate) receiver: Arc<Mutex<Receiver<Result<Event>>>>, // receive freesiwtch event
    pub(crate) connected: Arc<Mutex<bool>>,
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

        Err(EslError::ConnectionError(String::from("disconnected")))
    }

    pub async fn send(&self, command: &str) -> Result<()> {
        self.is_connected().await?;
        let sender = self.sender.clone();
        let sender = sender.lock().await;
        let command = format!("{}\n\n", command);
        match sender.send(command).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("send command error: {}", e);
                *self.connected.lock().await = false;
                return Err(EslError::ConnectionError(String::from(
                    "send command error",
                )));
            }
        };
        Ok(())
    }

    /// handle event
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

    /// return custom job-uuid
    pub async fn bgapi(&mut self, command: &str) -> Result<String> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let command = format!("bgapi {}\njob-uuid:{}", command, uuid);
        self.send(&command).await?;
        return Ok(uuid);
    }

    pub async fn api(&mut self, command: &str) -> Result<()> {
        let command = format!("api {}", command);
        self.send(&command).await
    }

    /// subscribe events
    /// only support json format
    pub async fn subscribe(&mut self, events: &[&str]) -> Result<()> {
        self.send(&format!("event json {}", events.join(" ")))
            .await?;
        Ok(())
    }

    pub async fn subscribe_all(&mut self) -> Result<()> {
        self.send("event json all").await?;
        Ok(())
    }

    pub async fn unsubscribe(&mut self, events: &[&str]) -> Result<()> {
        self.send(&format!("nixevent {}", events.join(" "))).await?;
        Ok(())
    }

    /// unsubscribe all
    pub async fn unsubscribe_all(&mut self) -> Result<()> {
        self.send("nixevent all").await?;
        Ok(())
    }
}
