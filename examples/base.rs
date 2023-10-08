use esl_rs::Esl;

#[tokio::main]
async fn main() {
    let mut conn = Esl::inbound("47.97.119.174:8021", "admin888")
        .await
        .unwrap();

    conn.handle(|evt| println!("evt: {:#?}", evt)).await;

    conn.send("api reloadxml").await.unwrap();

    let err = loop {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        if let Err(e) = conn.is_connected().await {
            break e;
        }
    };
    println!("err: {:?}", err);
}
