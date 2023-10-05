use env_logger::Env;
use esl_rs::{self, Esl};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let conn = Esl::inbound("47.97.119.174:8021", "admin888")
        .await
        .unwrap();

    let conn = Arc::new(Mutex::new(conn));

    log::debug!("send");
    let conn1 = conn.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(evt) = conn1.lock().await.recv().await {
                println!("evt: {:#?}", evt);
                let conn2 = conn1.clone();
                tokio::spawn(async move {
                    handler(evt, conn2).await;
                });
            }
        }
    });
    conn.lock()
        .await
        .send("event json CHANNEL_CREATE CHANNEL_DESTROY CHANNEL_ANSWER BACKGROUND_JOB")
        .await
        .unwrap();
    // 指定uuid
    let uuid = uuid::Uuid::new_v4().to_string();
    let r = conn
        .lock()
        .await
        .bgapi(&format!(
            "originate [ignore_early_media=true][origination_uuid={}]user/1001 &echo",
            uuid
        ))
        .await
        .unwrap();
    log::debug!("r: {:?}", r);
    tokio::time::sleep(std::time::Duration::from_secs(1000)).await;
}

async fn handler(evt: esl_rs::event::Event, conn: Arc<Mutex<esl_rs::conn::Conn>>) {
    println!("evt: {:#?}", evt);
    conn.lock().await.send("api uuid_kill all").await.unwrap();
}
