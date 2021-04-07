use actix_web::{HttpResponse, web::Path, Responder, Result, get, delete, post, put, web::{Data, Json, ServiceConfig, scope}};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::claim::{create_jwt, Claims};
use crate::models::user::User;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(self::register)
            .service(self::login)
            .service(self::get_users)
            .service(self::get_user)
            .service(self::add_user)
            .service(self::delete_user)
            .service(self::get_permissions));
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

#[get("")]
pub async fn get_users(data: Data<PgPool>) -> Result<HttpResponse> {
    match User::find_all(data.get_ref()).await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }
}

#[post("")]
pub async fn add_user(user: Json<UserRequest>, data: Data<PgPool>) -> Result<HttpResponse> {
    match User::create(user.into_inner(), data.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }    
}

#[get("/{id}")]
pub async fn get_user(Path(id): Path<Uuid>, data: Data<PgPool>) -> Result<HttpResponse> {
    match User::find_by_id(id, data.get_ref()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }
}

#[delete("/{id}")]
pub async fn delete_user(Path(id): Path<Uuid>, data: Data<PgPool>) -> Result<HttpResponse> {
    match User::delete(id, data.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }    
}

#[put("/{id}")]
pub async fn update_user(Path(id): Path<Uuid>, user: Json<UserRequest>, data: Data<PgPool>) -> Result<HttpResponse> {
    match User::update(id, user.into_inner(), data.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string()))
    }
}

#[get("/{id}/permissions")]
pub async fn get_permissions(Path(id): Path<Uuid>, data: Data<PgPool>) -> HttpResponse {
    match User::get_user_permissions(id, data.get_ref()).await {
        Ok(permissions) => HttpResponse::Ok().json(permissions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/login")]
pub async fn login(user: Json<UserRequest>, data: Data<PgPool>) -> Result<HttpResponse> {
    match User::is_signed_in(&user, data.as_ref()).await {
        Ok(db_user) => {
            match User::get_user_permissions(db_user.id, data.as_ref()).await {
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
