#![feature(once_cell)]

use actix_files::{Files, NamedFile};
use actix_web::{
    App,
    HttpRequest,
    HttpServer,
    Result as ActixResult,
    dev::ServiceRequest,
    middleware::Logger,
    web::{get, scope}
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use auth::claim::decode_jwt;
use env_logger::Env;
use sqlx::PgPool;
use std::{
    env,
    path::{Path, PathBuf},
};

mod auth;
mod backends;
mod config;
mod models;
mod routes;

pub async fn validate_jwt(req: ServiceRequest, auth: BearerAuth) -> ActixResult<ServiceRequest> {
    decode_jwt(auth.token())?;

    Ok(req)
}

async fn index(_: HttpRequest) -> ActixResult<NamedFile> {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/dist/index.html");
    let file = NamedFile::open(path)?;

    Ok(file.use_last_modified(true))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = config::Config::parse().expect("No configuration file found.");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let database_pool = PgPool::new(&database_url)
        .await
        .expect("Unable to create database pool.");
    let redis_host = env::var("REDIS_HOST").expect("Redis host is not set.");
    let backend = backends::RedisBackend::new(&redis_host);

    HttpServer::new(move || {
        let web_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/dist");

        App::new()
            .wrap(Logger::default())
	    .data(backend.clone())
	    .data(database_pool.clone())
            .service(
                scope("/api")
                    .configure(routes::auth::init)
                    .configure(routes::permissions::init)
                    .configure(routes::features::init)
                    .configure(routes::users::init),
            )
            .service(Files::new("/", web_dir).index_file("index.html"))
            .default_service(get().to(index))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
