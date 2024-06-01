use bytes::Bytes;
use clap::{Parser, Subcommand};
use s_redis::clients::Client;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::{env, str, time::Duration};
use tracing_appender::{non_blocking::WorkerGuard, rolling::Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[derive(Parser, Debug)]
#[clap(
    name = "mini-redis-cli",
    version,
    author,
    about = "Issue Redis commands"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(name = "port", long, default_value = "8888")]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {
        msg: Option<Bytes>,
    },
    Get {
        key: String,
    },
    Set {
        key: String,

        #[clap(value_parser = bytes_from_str)]
        value: Bytes,

        #[clap(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
}

#[tokio::main]
async fn main() -> s_redis::Result<()> {
    let _guard = init_log();

    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.host, cli.port);
    let mut client = Client::connect(addr).await.unwrap();

    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            if let Ok(str) = str::from_utf8(&value) {
                println!("\"{}\"", str);
            } else {
                println!("{:?}", value);
            }
        }
        Command::Get { key } => {
            let value = client.get(&key).await?;
            match value {
                Some(bytes) => {
                    if let Ok(str) = str::from_utf8(&bytes) {
                        println!("\"{}\"", str);
                    } else {
                        println!("{:?}", bytes);
                    }
                }
                None => {
                    println!("{:?}", value);
                }
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            let _rs = client.set(&key, value).await;
            println!("OK");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires),
        } => {
            client.set_expires(&key, value, expires).await?;
            println!("OK");
        }
    }

    Ok(())
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

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Result<Bytes, Infallible> {
    Ok(Bytes::from(src.to_string()))
}
