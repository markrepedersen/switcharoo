use actix_web::{HttpResponse, Responder, Result, post, web::{Data, Json, ServiceConfig, scope}};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::claim::{create_jwt, Claims};
use crate::models::{permission::Permission, user::User};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(self::register)
            .service(self::login));
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub username: String,
    pub permissions: Vec<String>,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(user: Json<UserRequest>, data: Data<PgPool>) -> Result<HttpResponse> {
    match User::is_signed_in(&user, data.as_ref()).await {
        Ok(db_user) => {
            match Permission::get_permissions_for_user(db_user.id, data.as_ref()).await {
                Ok(ref permissions) => {
                    let permissions: Vec<String> =
                        permissions.iter().map(|p| p.name.clone()).collect();
                    let claims = Claims::new(user.email.clone(), permissions.clone());
                    let token = create_jwt(claims)?;

                    Ok(HttpResponse::Ok().json(LoginResponse {
                        username: user.email.clone(),
                        permissions,
                        token,
                    }))
                }
                Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[post("/register")]
pub async fn register(user: Json<UserRequest>, data: Data<PgPool>) -> impl Responder {
    match User::create(user.into_inner(), data.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
