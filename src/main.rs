use std::sync::{Arc, Mutex};

use actix_web::{middleware::Logger, web::scope, App, HttpServer};
use redis::{Client, Connection, RedisResult};
use routes::features;

pub mod routes {
    pub mod features;
}

/**
The storage backend.
 */
#[derive(Clone)]
pub struct Backend {
    pub name: String,
    pub conn: Arc<Mutex<Connection>>,
}

impl Backend {
    /**
    This will create a connection to the backend.
    */
    pub fn new(name: String) -> RedisResult<Self> {
        let client = Client::open("redis://localhost")?;
        Ok(Self {
            name,
            conn: Arc::new(Mutex::new(client.get_connection()?)),
        })
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = String::from("localhost");
    let port: u16 = 8080;
    let backend = Backend::new("redis".to_string())?;

    println!("Starting server on {}:{}...", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(backend.clone())
            .service(
                scope("/features")
                    .service(features::set_toggle)
                    .service(features::is_toggled),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}
