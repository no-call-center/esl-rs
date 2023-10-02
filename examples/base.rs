use env_logger::Env;
use esl_rs::{self, Esl};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let mut conn = Esl::inbound("47.97.119.174:8021", "admin888")
        .await
        .unwrap();

    log::debug!("send");

    conn.send("event json CHANNEL_CREATE CHANNEL_DESTROY CHANNEL_ANSWER BACKGROUND_JOB").await.unwrap();
    // 指定uuid
    let uuid = uuid::Uuid::new_v4().to_string();
    let r = conn.bgapi(&format!("originate [ignore_early_media=true][origination_uuid={}]user/1001 &echo", uuid)).await.unwrap();
    log::debug!("r: {:?}", r);
    loop {
        if let Ok(evt) = conn.recv().await {
            println!("evt: {:#?}", evt);
        }
    }
}
