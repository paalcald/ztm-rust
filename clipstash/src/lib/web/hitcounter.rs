use rocket::futures::channel::mpsc::unbounded;
use tokio::runtime::Handle;

use crate::{service::ServiceError, data::DatabasePool};
use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc}, collections::HashMap};
use crate::domain::clip::field::ShortCode;

type HitStore = Arc<Mutex<HashMap<ShortCode, u64>>>;

#[derive(Debug, thiserror::Error)]
enum HitCountError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),
    #[error("communication error: {0}")]
    Channel(#[from] std::sync::mpsc::SendError<HitCountMsg>)
}

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32)
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>
}

impl HitCounter {
    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let store: HitStore = Arc::new(Mutex::new(HashMap::new()));
        });

        loop {
            
        }
    }

    pub fn hit(&self, shortcode: Shortcode, count: u64) {
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shortcode, count)) {
            eprintln!("hit count error: {}", e)
        }
    }
}