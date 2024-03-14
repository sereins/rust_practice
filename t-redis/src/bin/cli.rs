use std::env;

// use bytes::Bytes;
// use t_redis::clients::Client;
use tracing::info;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();

    info!("hello");

    println!("test log");
    // let mut client = Client::connect("127.0.0.1:8888").await.unwrap();
    //
    // let rs = client.ping(Some(Bytes::from("Ping".as_bytes()))).await;
    //
    // println!("ping return {:?}", rs);
}
