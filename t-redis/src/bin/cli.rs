use bytes::Bytes;
use std::env;
use t_redis::clients::Client;
use tracing_appender::{non_blocking::WorkerGuard, rolling::Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() {
    let _guard = init_log();
    let mut client = Client::connect("127.0.0.1:8888").await.unwrap();

    let rs = client.ping(Some(Bytes::from("Ping".as_bytes()))).await;

    println!("ping return {:?}", rs);
}

fn init_log() -> Option<WorkerGuard> {
    // 是否需要输出到日志
    let write_file = env::var("WRITE_FILE").unwrap_or("false".to_string());

    let stdout = std::io::stdout;

    let subscriber = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG);
    if write_file == "true" {
        let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_suffix("log")
            .build("./logs")
            .expect("build log file appender errro");

        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        subscriber
            .with_writer(stdout.and(non_blocking))
            .with_ansi(false)
            .init();
        Some(guard)
    } else {
        subscriber.with_writer(stdout).init();
        None
    }
}
