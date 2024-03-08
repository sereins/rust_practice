use std::future::Future;
use std::{sync::Arc, time::Duration};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc, Semaphore},
    time,
};

use crate::{
    connection::Connection,
    db::{Db, DbDropGuard},
    Command, Result, Shutdown,
};

const MAX_CONNECTIONS: usize = 250;

struct Listener {
    db_holder: DbDropGuard,

    listener: TcpListener,

    limit_connections: Arc<Semaphore>,

    notify_shutdown: broadcast::Sender<()>,

    shutdown_complete_tx: mpsc::Sender<()>,
}

struct Handler {
    db: Db,
    connection: Connection,
    shutdown: Shutdown,
}

impl Listener {
    async fn run(&self) -> Result<()> {
        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();

            let socket = self.accept().await?;

            let mut handler = Handler {
                db: self.db_holder.db(),
                connection: Connection::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    // error!(cause = ?err, "connection error");
                    println!("cause err{:?}", err);
                };
                drop(permit);
            });
        }
    }

    async fn accept(&self) -> Result<TcpStream> {
        let mut backoff = 1;

        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(error) => {
                    if backoff > 64 {
                        return Err(error.into());
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;

            backoff *= 2;
        }
    }
}

impl Handler {
    async fn run(&mut self) -> Result<()> {
        while !self.shutdown.is_shutdown() {
            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res?,
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };

            let frame = match maybe_frame {
                Some(frame) => frame,
                None => return Ok(()),
            };

            let cmd = Command::from_frame(frame)?;

            cmd.apply(&self.db, &mut self.connection, &mut self.shutdown)
                .await?;
        }

        Ok(())
    }
}

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

    let server = Listener {
        listener,
        db_holder: DbDropGuard::new(),
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        notify_shutdown,
        shutdown_complete_tx,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                // error!(cause = %err, "failed to accept");
                println!("err:{:?}",err);
            }
        }
        _ = shutdown => {
            // info!("shutting down");
            println!("shutting down");
        }
    }

    let Listener {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;
}
