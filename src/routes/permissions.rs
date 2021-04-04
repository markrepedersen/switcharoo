use crate::{models::permission::Permission, validate_jwt};
use actix_web::{HttpResponse, delete, get, post, web::Json, web::Path, web::{Data, ServiceConfig, scope}};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct PermissionRequest {
    pub name: String,
}

#[get("/permissions")]
pub async fn get_permissions(data: Data<PgPool>) -> HttpResponse {
    match Permission::get_permissions(data.as_ref()).await {
        Ok(permissions) => HttpResponse::Ok().json(permissions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/permissions")]
pub async fn add_permissions(
    permission: Json<PermissionRequest>,
    data: Data<PgPool>,
) -> HttpResponse {
    match Permission::add_permission(permission.name.clone(), data.as_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/permissions/{name}")]
pub async fn remove_permissions(Path(name): Path<String>, data: Data<PgPool>) -> HttpResponse {
    match Permission::remove_permission(name, data.as_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/permissions")
	    .wrap(HttpAuthentication::bearer(validate_jwt))
	    .service(self::get_permissions)
	    .service(self::add_permissions)
	    .service(self::remove_permissions));
}
