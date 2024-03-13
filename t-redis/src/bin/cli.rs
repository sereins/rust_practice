use bytes::Bytes;
use t_redis::clients::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::connect("127.0.0.1:8888").await.unwrap();

    let rs = client.ping(Some(Bytes::from("Ping".as_bytes()))).await;

    println!("ping return {:?}", rs);
}
