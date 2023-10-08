use env_logger::Env;
use esl_rs::event::Event;
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

    conn.lock().await.subscribe_all().await.unwrap();

    // custorm uuid
    let uuid = uuid::Uuid::new_v4().to_string();
    let r = conn
        .lock()
        .await
        .bgapi(&format!(
            "originate [call_id=123456][ignore_early_media=true][origination_uuid=111111]user/1001 &park",
        ))
        .await
        .unwrap();

    log::debug!("r: {:?}", r);

    let result = run!(conn);
    log::error!("result: {:?}", result);
}

async fn handler(evt: esl_rs::event::Event, conn: Arc<Mutex<esl_rs::conn::Conn>>) {
    match evt {
        Event::ChannelCreate(v) => {
            log::debug!("ChannelCreate : {:#?}", v);
        }
        Event::ChannelDestroy(v) => {
            log::debug!("ChannelDestroy : {:#?}", v);
        }
        Event::ChannelAnswer(v) => {
            log::debug!("ChannelAnswer : {:#?}", v);
            let call_id = v.get_var("call_id");
            log::info!("call_id: {:?}", call_id);

            if let Some(leg) = v.get_var("origination_uuid") {
                if leg == "111111" {
                    let r = conn
                    .lock()
                    .await
                    .bgapi(&format!(
                        "originate [call_id=123456][ignore_early_media=true][origination_uuid=222222]user/1002 &park",
                    ))
                    .await
                    .unwrap();
                } else if leg == "222222" {
                    // bridge
                    let r = conn
                        .lock()
                        .await
                        .api(&format!("uuid_bridge {} {} both", leg, "111111"))
                        .await
                        .unwrap();
                }
            }
        }
        Event::ChannelHangupComplete(v) => {
            log::debug!("ChannelHangupComplete : {:#?}", v);
        }
        v => {
            log::debug!("other : {:#?}", v);
        }
    }
    // conn.lock().await.send("api status").await;
}
