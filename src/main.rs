use actix_redis::RedisSession;
use actix_web::{middleware::Logger, web::scope, App, HttpServer};
use backends::Backend;
use config::Config;
use dotenv;
use env_logger::Env;
use rand::Rng;
use routes::{auth, features, web};
use sqlx::PgPool;
use std::env;

mod backends;
mod config;
mod models;
mod routes;

fn gen_key() -> [u8; 32] {
    rand::thread_rng().gen::<[u8; 32]>()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::parse().expect("No configuration file found.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let database_pool = PgPool::new(&database_url)
        .await
        .expect("Unable to create database pool.");

    let redis_host = env::var("REDIS_HOST").expect("Redis host is not set.");
    let redis_port = env::var("REDIS_PORT").expect("Redis port is not set.");
    let redis_key = gen_key();

    let backend = Backend::new(&redis_host);

    HttpServer::new(move || {
        let session =
            RedisSession::new(format!("{}:{}", redis_host.clone(), redis_port), &redis_key)
                .ttl(1800)
                .cookie_name("ff_session")
                .cookie_max_age(None);

        App::new()
            .wrap(session)
            .wrap(Logger::default())
            .data(backend.clone())
            .data(database_pool.clone())
            .configure(web::init)
            .service(
                scope("/api")
                    .configure(auth::init)
                    .configure(features::init),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
