use env_logger::Env;
use esl_rs::run;
use esl_rs::{self, Esl};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let target = Box::new(File::create("./tmp.log").expect("Can't create file"));
    env_logger::Builder::from_env(Env::default().default_filter_or("error"))
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Always)
        .target(env_logger::Target::Pipe(target))
        .init();

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
            "CHANNEL_HUGUP",
            "CHANNEL_HANGUP_COMPLETE",
            "BACKGROUND_JOB",
        ])
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
    log::debug!("r: {:?}", r);

    let result = run!(conn);
    log::error!("result: {:?}", result);
}

async fn handler(evt: esl_rs::event::Event, conn: Arc<Mutex<esl_rs::conn::Conn>>) {
    println!("evt: {:#?}", evt);
    conn.lock().await.send("api status").await;
}
