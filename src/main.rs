#![feature(once_cell)]

use actix_files::{Files, NamedFile};
use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Result as ActixResult,
    dev::ServiceRequest,
    middleware::Logger,
    post,
    web::Data,
    web::Json,
    web::{get, scope}
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use auth::claim::{Claims, create_jwt, decode_jwt};
use env_logger::Env;
use models::user::User;
use routes::users::UserRequest;
use sqlx::PgPool;
use uuid::Uuid;
use std::{
    env,
    path::{Path, PathBuf},
};
use serde::{Serialize, Deserialize};

mod auth;
mod backends;
mod config;
mod models;
mod routes;

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub username: String,
    pub permissions: Vec<String>,
    pub token: String,
}

pub async fn validate_jwt(req: ServiceRequest, auth: BearerAuth) -> ActixResult<ServiceRequest> {
    decode_jwt(auth.token())?;

    Ok(req)
}

#[post("/login")]
pub async fn login(user: Json<UserRequest>, data: Data<PgPool>) -> ActixResult<HttpResponse> {
    Ok(match User::is_signed_in(&user, data.as_ref()).await {
        Ok(db_user) => match User::get_user_permissions(db_user.id, data.as_ref()).await {
            Ok(ref permissions) => {
                let permissions: Vec<String> = permissions.iter().map(|p| p.name.clone()).collect();
                let claims =
                    Claims::new(user.email.clone(), db_user.tenant_id, permissions.clone());
                let token = create_jwt(claims)?;

                HttpResponse::Ok().json(LoginResponse {
                    username: user.email.clone(),
                    permissions,
                    token,
                })
            }
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(_) => HttpResponse::Unauthorized().finish(),
    })
}

#[post("/register")]
pub async fn register(
    user: Json<UserRequest>,
    data: Data<PgPool>,
) -> ActixResult<HttpResponse> {
    // TODO: assign tenant_id
    Ok(
        match User::create(user.into_inner(), Uuid::from_u128(1), data.get_ref()).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    )
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
            .service(
                scope("/api")
		    .data(backend.clone())
		    .data(database_pool.clone())
		    .service(login)
		    .service(register)
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
