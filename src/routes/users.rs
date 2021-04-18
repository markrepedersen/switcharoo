use actix_web::{
    delete, get, post, put,
    web::Path,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Result,
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::auth::validate_jwt;

use crate::auth::claim::decode_jwt;
use crate::models::user::User;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .service(self::get_users)
            .service(self::get_user)
            .service(self::add_user)
            .service(self::delete_user)
            .service(self::get_permissions),
    );
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

#[get("")]
pub async fn get_users(data: Data<PgPool>, auth: BearerAuth) -> Result<HttpResponse> {
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;

    Ok(match User::find_all(tenant_id, data.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[post("")]
pub async fn add_user(
    user: Json<UserRequest>,
    data: Data<PgPool>,
    auth: BearerAuth,
) -> Result<HttpResponse> {
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;

    Ok(
        match User::create(user.into_inner(), tenant_id, data.get_ref()).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    )
}

#[get("/{id}")]
pub async fn get_user(id: Path<Uuid>, data: Data<PgPool>) -> Result<HttpResponse> {
    let id = id.into_inner();

    Ok(match User::find_by_id(id, data.get_ref()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[delete("/{id}")]
pub async fn delete_user(id: Path<Uuid>, data: Data<PgPool>) -> Result<HttpResponse> {
    let id = id.into_inner();

    Ok(match User::delete(id, data.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[put("/{id}")]
pub async fn update_user(
    id: Path<Uuid>,
    user: Json<UserRequest>,
    data: Data<PgPool>,
) -> Result<HttpResponse> {
    let id = id.into_inner();

    Ok(
        match User::update(id, user.into_inner(), data.get_ref()).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
    )
}

#[get("/{id}/permissions")]
pub async fn get_permissions(id: Path<Uuid>, data: Data<PgPool>) -> HttpResponse {
    let id = id.into_inner();

    match User::get_user_permissions(id, data.get_ref()).await {
        Ok(permissions) => HttpResponse::Ok().json(permissions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
