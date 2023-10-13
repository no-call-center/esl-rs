use esl_rs::run;
use esl_rs::{self, Esl};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set the global default subscriber");

    let mut conns = HashMap::new();

    let fs1 = "47.97.119.174:8021";
    loop {
        let conn = match Esl::inbound(fs1, "admin888").await {
            Ok(conn) => conn,
            Err(e) => {
                error!("connect error: {}", e);
                continue;
            }
        };

        conns.insert(fs1, Arc::new(Mutex::new(conn)));

        debug!("send");

        let conn = match conns.get(fs1) {
            Some(conn) => conn.clone(),
            None => break,
        };
        let conn1 = conn.clone();
        conn.lock()
            .await
            .handle(move |evt| {
                let conn1 = conn1.clone();
                tokio::spawn(async move {
                    handler(evt, conn1).await;
                });
            })
            .await;

        conn.lock()
        .await
        .send(
            "event json CHANNEL_CREATE CHANNEL_DESTROY CHANNEL_ANSWER CHANNEL_HUGUP BACKGROUND_JOB",
        )
        .await
        .unwrap();

        // custorm uuid
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
        debug!("r: {:?}", r);

        let result = run!(conn);
        error!("result: {:?}", result);
        conns.remove(fs1);
    }
}

async fn handler(evt: esl_rs::event::Event, conn: Arc<Mutex<esl_rs::conn::Conn>>) {
    println!("evt: {:#?}", evt);
}
