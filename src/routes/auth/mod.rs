use actix_web::{post, web::Data, web::Json, web::ServiceConfig, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(user: Json<UserRequest>, data: Data<PgPool>) -> impl Responder {
    match User::has_password(user.into_inner(), data.get_ref()).await {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::Unauthorized().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/register")]
pub async fn register(user: Json<UserRequest>, data: Data<PgPool>) -> impl Responder {
    match User::create(user.into_inner(), data.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
