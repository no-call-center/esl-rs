mod error;
mod conn;
mod event;

use crate::error::EslError;
use futures::{SinkExt, StreamExt};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, WriteHalf},
    net::{TcpStream, ToSocketAddrs},
    sync::{
        oneshot::{channel, Sender},
        Mutex,
    },
};
pub struct Esl;

impl Esl {
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

    pub async fn inbound(
        addr: impl ToSocketAddrs,
        password: impl ToString,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(addr).await?;

        let (mut read_half, mut write_half) = stream.into_split();

        tokio::spawn(async move {
            let mut all_buf = Vec::new();
            loop {
                let mut buf = [0; 5000];
                let n = read_half.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                all_buf.extend_from_slice(&buf[..n]);
                // 找头结束的地方
                while let Some(header_end) = Self::get_header_end(&all_buf) {
                    let header = &all_buf[..(header_end - 1)];

                    let headers = Self::parse_header(header);
                    // println!("headers: {:?}", headers);
                    let header = String::from_utf8_lossy(header).to_string();

                    // 如果key有 Content-Length，则获取后续长度为body
                    let body = if let Some(content_length) = headers.get("Content-Length") {
                        let content_length = content_length.trim().parse::<usize>().unwrap();
                        // 如果没接收完，继续接收
                        if content_length > all_buf.len() - header_end - 1 {
                            break;
                        }
                        let body = String::from_utf8_lossy(
                            &all_buf[(header_end + 1)..(header_end + 1 + content_length)],
                        )
                        .to_string();
                        all_buf = all_buf[(header_end + 1 + content_length)..].to_vec();
                        Some(body)
                    } else {
                        all_buf = all_buf[(header_end + 1)..].to_vec();
                        None
                    };

                    // println!("header: {:?}, ", String::from_utf8_lossy(header));
                    println!("header: {}", header);
                    println!("body: {}", body.unwrap_or_default());
                }
            }
        });

        write_half
            .write(format!("auth {}\n\n", password.to_string()).as_bytes())
            .await?;

        // 订阅所有事件
        write_half.write(b"event json ALL\n\n").await?;

        // 发起呼叫 1000 和 1001
        write_half
            .write(b"bgapi originate user/1001 &echo\n\n")
            .await?;

        // 等待100秒
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inbound() {
        Esl::inbound("47.97.119.174:8021", "admin888")
            .await
            .unwrap();
    }
}
