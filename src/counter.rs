use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use tokio::sync::watch;

#[derive(Clone)]
pub struct Counter {
    value: Arc<AtomicU64>,
    tx: Arc<watch::Sender<u64>>,
}

impl Counter {
    pub fn new() -> Self {
        let (tx, _rx) = watch::channel(0);

        Self {
            value: Arc::new(AtomicU64::new(0)),
            tx: Arc::new(tx),
        }
    }

    pub fn increment(&self) -> u64 {
        let new_value = self.value.fetch_add(1, Ordering::SeqCst) + 1;
        let _ = self.tx.send(new_value);

        new_value
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }
}
