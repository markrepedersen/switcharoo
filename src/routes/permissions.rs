use crate::models::permission::Permission;
use actix_web::{
    delete, get, post, put,
    web::Json,
    web::Path,
    web::{scope, Data, ServiceConfig},
    HttpResponse,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::auth::validate_jwt;

#[derive(Serialize, Deserialize)]
pub struct PermissionRequest {
    pub name: String,
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/permissions")
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .service(self::get_permission)
            .service(self::get_permissions)
            .service(self::add_permissions)
            .service(self::remove_permissions),
    );
}

#[get("")]
pub async fn get_permissions(data: Data<PgPool>) -> HttpResponse {
    match Permission::get_permissions(data.as_ref()).await {
        Ok(permissions) => HttpResponse::Ok().json(permissions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("")]
pub async fn add_permissions(
    permission: Json<PermissionRequest>,
    data: Data<PgPool>,
) -> HttpResponse {
    match Permission::add_permission(permission.name.clone(), data.as_ref()).await {
        Ok(permission) => HttpResponse::Ok().json(permission),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_permission(id: Path<i32>, data: Data<PgPool>) -> HttpResponse {
    let id = id.into_inner();

    match Permission::get_permission(id, data.as_ref()).await {
        Ok(permission) => HttpResponse::Ok().json(permission),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_permissions(id: Path<i32>, data: Data<PgPool>) -> HttpResponse {
    let id = id.into_inner();

    match Permission::update_permission(id, data.as_ref()).await {
        Ok(permission) => HttpResponse::Ok().json(permission),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
pub async fn remove_permissions(id: Path<i32>, data: Data<PgPool>) -> HttpResponse {
    let id = id.into_inner();

    match Permission::remove_permission(id, data.as_ref()).await {
        Ok(permission) => HttpResponse::Ok().json(permission),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
