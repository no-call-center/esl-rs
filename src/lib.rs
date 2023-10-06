pub mod conn;
pub mod error;
pub mod event;

use crate::error::EslError;
use conn::Conn;
use error::Result;
use event::{get_header_end, parse_header, Event};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::{mpsc::channel, Mutex},
};

pub struct Esl;

impl Esl {
    pub async fn inbound(addr: impl ToSocketAddrs, password: impl ToString) -> Result<Conn> {
        let (event_tx, event_rx) = channel::<Event>(1000);
        let (command_tx, mut command_rx) = channel::<String>(1000);
        let command_tx = Arc::new(Mutex::new(command_tx));
        let command_tx1 = command_tx.clone();
        let password = password.to_string();
        let stream = TcpStream::connect(addr).await?;
        let (mut read_half, mut write_half) = stream.into_split();
        let authed = Arc::new(Mutex::new(false));
        let auth_err = Arc::new(Mutex::new(Result::<()>::Err(EslError::AuthFailed)));
        let conn = Conn::new(command_tx, Arc::new(Mutex::new(event_rx)));
        let authed1 = authed.clone();
        let auth_err1 = auth_err.clone();
        // receive all event
        tokio::spawn(async move {
            let mut all_buf = Vec::new();
            loop {
                let mut buf = [0; 10240];
                let n = read_half.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                all_buf.extend_from_slice(&buf[..n]);

                let header_end = match get_header_end(&all_buf) {
                    Some(header_end) => header_end,
                    None => continue,
                };

                let header = &all_buf[..header_end-1];

                let headers = parse_header(header);
                let header = String::from_utf8_lossy(header).to_string();

                println!("all_buf: {:?}", String::from_utf8_lossy(&all_buf));
                let body = if let Some(content_length) = headers.get("Content-Length") {
                    let content_length = content_length.trim().parse::<usize>().unwrap();
                    // if recive data less than content_length, continue
                    if content_length > all_buf.len() - header_end {
                        continue;
                    }
                    let body = String::from_utf8_lossy(
                        &all_buf[(header_end)..(header_end + content_length)],
                    )
                    .to_string();
                    all_buf = all_buf[(header_end + content_length)..].to_vec();
                    Some(body)
                } else {
                    all_buf = all_buf[(header_end)..].to_vec();
                    None
                };

                if header.contains("auth/request") {
                    command_tx1
                        .lock()
                        .await
                        .send(format!("auth {}\n\n", password))
                        .await
                        .unwrap();
                    continue;
                } else if header.contains("Reply-Text: +OK accepted")
                    && header.contains("command/reply")
                {
                    let mut auth_err = auth_err1.lock().await;
                    *auth_err = Ok(());
                    let mut authed = authed1.lock().await;
                    *authed = true;
                    log::debug!("auth success");
                } else if header.contains("invalid") && header.contains("command/reply") {
                    let mut authed = authed1.lock().await;
                    if !*authed {
                        *authed = true;

                        let mut auth_err = auth_err1.lock().await;
                        *auth_err = Err(EslError::AuthFailed);
                    }
                }

                log::debug!("raw header: {:?}", header);
                log::debug!("raw body: {:?}", body);
                let evt = Event::new(headers, body);
                event_tx.send(evt).await.unwrap();
            }
        });

        tokio::spawn(async move {
            log::debug!("send command start");
            loop {
                let command = command_rx.recv().await.unwrap();

                write_half.write(command.as_bytes()).await.unwrap();
                log::debug!("send command: {}", command);
            }
        });

        loop {
            {
                let authed = authed.lock().await;
                if *authed {
                    break;
                }
            }
            // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        let auth_err = auth_err.lock().await;
        if let Err(e) = auth_err.clone() {
            return Err(e);
        }
        log::debug!("认证成功");
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inbound() {
        env_logger::init();

        let conn = Esl::inbound("47.97.119.174:8021", "admin888")
            .await
            .unwrap();

        let conn1 = conn.clone();
        // tokio::spawn(async move {
        //     loop {
        //         if let Ok(evt) = conn1.lock().await.recv().await {
        //             println!("evt: {:#?}", evt);
        //         }
        //     }
        // });

        // log::debug!("send");
        // conn.lock().await.send("event json ALL").await.unwrap();
        // conn.lock()
        //     .await
        //     .send("bgapi originate user/1001 &echo")
        //     .await
        //     .unwrap();
    }
}
