use std::{
    env,
    sync::{Arc, Mutex},
};

use actix_web::{middleware::Logger, web::scope, App, HttpServer};
use config::Config;
use dotenv;
use env_logger::Env;
use redis::{Client, Connection, RedisResult};
use routes::{auth, features, web};
use sqlx::PgPool;

mod backends;
mod config;
mod models;
mod routes;

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
    pub fn new(connection_str: String, name: String) -> RedisResult<Self> {
        let client = Client::open(connection_str)?;

        Ok(Self {
            name,
            conn: Arc::new(Mutex::new(client.get_connection()?)),
        })
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::parse().expect(
        "No configuration file found. Please create a 'config.toml' file in the root folder.",
    );
    let backend = Backend::new("redis://localhost".to_string(), "redis".to_string())?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPool::new(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(backend.clone())
            .data(pool.clone())
            .service(
                scope("/api")
                    .configure(auth::init)
                    .configure(features::init)
                    .configure(web::init),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
