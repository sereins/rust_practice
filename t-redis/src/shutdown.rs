use tokio::sync::broadcast;

pub(crate) struct Shutdown {
    is_shutdown: bool,
    notify: broadcast::Receiver<()>,
}

impl Shutdown {
    pub(crate) fn new(noitfy: broadcast::Receiver<()>) -> Self {
        Self {
            is_shutdown: false,
            notify: noitfy,
        }
    }

    pub(crate) fn is_shutdown(&self) -> bool {
        self.is_shutdown
    }

    pub(crate) async fn recv(&mut self) {
        if self.is_shutdown {
            return;
        }

        let _ = self.notify.recv().await;

        self.is_shutdown = true;
    }
}
