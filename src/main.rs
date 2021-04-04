#![feature(once_cell)]

use actix_files::Files;
use actix_web::{
    dev::ServiceRequest, middleware::Logger, web::scope, App, HttpServer, Result as ActixResult,
};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use env_logger::Env;
use sqlx::PgPool;
use std::{env, path};

use backends::Backend;
use config::Config;

mod auth;
mod backends;
mod config;
mod models;
mod routes;

pub async fn validate_jwt(req: ServiceRequest, credentials: BearerAuth) -> ActixResult<ServiceRequest> {
    let claims = auth::claim::decode_jwt(credentials.token())?;

    req.attach(claims.permissions.iter().map(|p| p.to_string()).collect());

    Ok(req)
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
    let backend = Backend::new(&redis_host);

    HttpServer::new(move || {
	let web_dir = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/dist");
	
        App::new()
            .wrap(Logger::default())
            .data(backend.clone())
            .data(database_pool.clone())
            .service(
                scope("/api")
                    .configure(routes::permissions::init)
                    .configure(routes::features::init)
		    .configure(routes::users::init)
            )
	    .service(Files::new("/", web_dir).index_file("index.html"))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
