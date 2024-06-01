use bytes::Bytes;
use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex},
};
use tokio::{
    sync::{broadcast, Notify},
    time::{self, Duration, Instant},
};

#[derive(Debug)]
pub(crate) struct DbDropGuard {
    db: Db,
}

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug)]
struct State {
    // key-value 类型的data
    entries: HashMap<String, Entry>,

    // 发布订阅数据
    _pub_sub: HashMap<String, broadcast::Sender<Bytes>>,

    // 过期维护
    expirations: BTreeSet<(Instant, String)>,

    // 关闭
    shutdown: bool,
}

#[derive(Debug)]
struct Entry {
    // 数据
    data: Bytes,
    // 过期的时间
    expires_at: Option<Instant>,
}

impl DbDropGuard {
    pub(crate) fn new() -> Self {
        Self { db: Db::new() }
    }

    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Drop for DbDropGuard {
    fn drop(&mut self) {
        self.db.showdown_purge_tasks();
    }
}

impl Db {
    pub(crate) fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                _pub_sub: HashMap::new(),
                expirations: BTreeSet::new(),
                shutdown: false,
            }),
            background_task: Notify::new(),
        });

        // 注册过期的任务
        tokio::spawn(purge_expired_tasks(shared.clone()));

        Db { shared }
    }

    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|entry| entry.data.clone())
    }

    pub(crate) fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();
        let mut notify = false;

        let expired_at = expire.map(|duration| {
            let when = Instant::now() + duration;

            notify = state
                .next_expiration()
                .map(|expiration| expiration > when)
                .unwrap_or(true);
            when
        });

        // insert into value to HashMap
        let prev = state.entries.insert(
            key.clone(),
            Entry {
                data: value,
                expires_at: expired_at,
            },
        );

        // 防止冲突,移除已经存在过的key
        if let Some(prev) = prev {
            if let Some(expired_at) = prev.expires_at {
                state.expirations.remove(&(expired_at, key.clone()));
            }
        }

        if let Some(when) = expired_at {
            state.expirations.insert((when, key));
        }
        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }
    }

    fn showdown_purge_tasks(&self) {
        let mut state = self.shared.state.lock().unwrap();
        state.shutdown = true;

        drop(state);

        self.shared.background_task.notify_one();
    }
}

impl Shared {
    //  移除过期的key,并返回下一个要过期的key
    fn purge_expired_key(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();

        if state.shutdown {
            return None;
        }

        let state = &mut *state;
        let now = Instant::now();

        while let Some(&(when, ref key)) = state.expirations.iter().next() {
            if when > now {
                return Some(when);
            }

            state.entries.remove(key);
            state.expirations.remove(&(when, key.clone()));
        }
        None
    }

    fn is_shotdown(&self) -> bool {
        self.state.lock().unwrap().shutdown
    }
}

impl State {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations
            .iter()
            .next()
            .map(|expiration| expiration.0)
    }
}

async fn purge_expired_tasks(shared: Arc<Shared>) {
    while !shared.is_shotdown() {
        if let Some(when) = shared.purge_expired_key() {
            tokio::select! {
                _= time::sleep_until(when) =>{},
                _= shared.background_task.notified()=>{}
            }
        } else {
            shared.background_task.notified().await;
        }
    }
}
