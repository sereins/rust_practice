use tokio::{net::TcpListener, signal};

use t_redis::{server, Result};
#[tokio::main]
pub async fn main() -> crate::Result<()> {
    println!("server start");

    let addr = "127.0.0.1:8888";
    let listener = TcpListener::bind(addr).await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}
