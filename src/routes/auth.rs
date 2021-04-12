use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    get, post,
    web::Data,
    web::Json,
    web::ServiceConfig,
    HttpResponse, Result as ActixResult,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::claim::{create_jwt, Claims};
use crate::models::user::User;
use crate::routes::users::UserRequest;

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub username: String,
    pub permissions: Vec<String>,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub token: String,
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(get_access_token);
}

#[get("/access_token")]
pub async fn get_access_token(
    user: Json<UserRequest>,
    data: Data<PgPool>,
) -> ActixResult<HttpResponse> {
    // TODO: set different expiry for token endpoints
    issue_token(&user, data.as_ref()).await
}

#[post("/login")]
pub async fn login(user: Json<UserRequest>, data: Data<PgPool>) -> ActixResult<HttpResponse> {
    issue_token(&user, data.as_ref()).await
}

#[post("/register")]
pub async fn register(user: Json<UserRequest>, data: Data<PgPool>) -> ActixResult<HttpResponse> {
    // TODO: assign tenant_id
    let user = User::create(user.into_inner(), Uuid::from_u128(1), data.get_ref())
        .await
        .map_err(|e| ErrorUnauthorized(e))?;

    Ok(HttpResponse::Ok().json(user))
}

async fn issue_token(payload: &UserRequest, data: &PgPool) -> ActixResult<HttpResponse> {
    let user = User::is_signed_in(payload, data)
        .await
        .map_err(|e| ErrorUnauthorized(e))?;
    let permissions = User::get_user_permissions(user.id, data)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;
    let claims = Claims::new(
        payload.email.clone(),
        user.tenant_id,
        permissions.iter().map(|p| p.name.clone()).collect(),
    );
    let token = create_jwt(claims).map_err(|e| ErrorBadRequest(e))?;

    Ok(HttpResponse::Ok().json(AccessTokenResponse { token }))
}
