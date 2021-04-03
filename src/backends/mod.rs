use std::sync::{Arc, Mutex};

use ::redis::{Client, Connection};

pub mod redis;

/**
The storage backend.
 */
#[derive(Clone)]
pub struct Backend {
    pub conn: Arc<Mutex<Connection>>,
}

impl Backend {
    /**
    This will create a connection to the backend.
    */
    pub fn new(host: &String) -> Self {
        let client = Client::open(format!("redis://{}", host)).expect("Redis URL parsing failed.");

        Self {
            conn: Arc::new(Mutex::new(
                client
                    .get_connection()
                    .expect("Unable to create connection to Redis."),
            )),
        }
    }
}
