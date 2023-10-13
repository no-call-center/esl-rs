use esl_rs::event::Event;
use esl_rs::run;
use esl_rs::{self, Esl};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let file = File::create("app.log").unwrap();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_writer(move || {
            // 将日志写入文件
            let file_clone = file.try_clone().expect("Failed to clone file");
            Box::new(file_clone) as Box<dyn Write + Send>
        })
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set the global default subscriber");

    let fs1 = "47.97.119.174:8021";

    let conn = Esl::inbound(fs1, "admin888").await.unwrap();

    let conn = Arc::new(Mutex::new(conn));

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
        .subscribe(&[
            "CHANNEL_CREATE",
            "CHANNEL_DESTROY",
            "CHANNEL_ANSWER",
            "CHANNEL_HANGUP",
            "CHANNEL_HANGUP_COMPLETE",
            "BACKGROUND_JOB",
        ])
        .await
        .unwrap();

    // custorm uuid
    let uuid = uuid::Uuid::new_v4().to_string();

    for i in 0..1 {
        let r = conn
            .lock()
            .await
            .bgapi(&format!(
                "originate [origination_uuid={}]user/1004 &echo",
                uuid
            ))
            .await
            .unwrap();

        info!("r: {:?}", r);

        // 执行 eavesdrop 监听指定通话
        let r = conn
            .lock()
            .await
            .bgapi(&format!("eavesdrop {}", uuid))
            .await
            .unwrap();
    }

    let result = run!(conn);
    error!("result: {:?}", result);
}

async fn handler(evt: esl_rs::event::Event, conn: Arc<Mutex<esl_rs::conn::Conn>>) {
    info!("other : {:#?}", evt);
    // conn.lock().await.send("api status").await;
}
