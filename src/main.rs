#![feature(once_cell)]

use actix_web::{App, HttpServer, middleware::Logger, web::scope};
use env_logger::Env;
use sqlx::PgPool;
use std::env;

mod auth;
mod backends;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let database_pool = PgPool::new(&database_url)
        .await
        .expect("Unable to create database pool.");
    let redis_host = env::var("REDIS_HOST").expect("Redis host is not set.");
    let backend = backends::RedisBackend::new(&redis_host);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(backend.clone())
            .data(database_pool.clone())
            .service(
                scope("/api")
                    .configure(routes::permissions::init)
                    .configure(routes::features::init)
                    .configure(routes::auth::init)
                    .configure(routes::users::init),
            )
            .service(routes::web::dist)
    })
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
}
