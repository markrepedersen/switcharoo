use actix_web::{dev::ServiceRequest, Result as ActixResult};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    get, post,
    web::{Data, Json, ServiceConfig},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::claim::{create_jwt, decode_jwt, Claims, JWT_DEFAULT_EXPIRATION};
use crate::models::user::User;
use crate::routes::users::UserRequest;

#[derive(Serialize, Deserialize)]
pub struct AccessTokenRequest {
    pub user: UserRequest,
    pub expiry: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub token: String,
    pub permissions: Vec<String>,
    pub expiry: i64,
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(get_access_token);
}

pub async fn validate_jwt(req: ServiceRequest, auth: BearerAuth) -> ActixResult<ServiceRequest> {
    decode_jwt(auth.token())?;

    Ok(req)
}

#[get("/access_token")]
/// Acquire a READ-ONLY access token.
pub async fn get_access_token(
    payload: Json<AccessTokenRequest>,
    data: Data<PgPool>,
) -> ActixResult<HttpResponse> {
    issue_token(&payload.into_inner(), data.as_ref()).await
}

#[post("/login")]
pub async fn login(payload: Json<UserRequest>, data: Data<PgPool>) -> ActixResult<HttpResponse> {
    let config = AccessTokenRequest {
        user: payload.into_inner(),
        expiry: JWT_DEFAULT_EXPIRATION,
    };

    issue_token(&config, data.as_ref()).await
}

#[post("/register")]
// TODO: Tenant ID is currently hard coded to 1 for all users.
pub async fn register(user: Json<UserRequest>, data: Data<PgPool>) -> ActixResult<HttpResponse> {
    let user = User::create(user.into_inner(), Uuid::from_u128(1), data.get_ref())
        .await
        .map_err(|e| ErrorUnauthorized(e))?;

    Ok(HttpResponse::Ok().json(user))
}

async fn issue_token(config: &AccessTokenRequest, data: &PgPool) -> ActixResult<HttpResponse> {
    let user = User::exists(&config.user, data)
        .await
        .map_err(|e| ErrorUnauthorized(e))?;

    if let Some(user) = user {
        let permissions: Vec<String> = User::get_user_permissions(user.id, data)
            .await
            .map_err(|e| ErrorInternalServerError(e))?
            .iter()
            .map(|p| p.name.clone())
            .collect();

        let claims = Claims::new(
            config.user.email.clone(),
            config.expiry,
            user.tenant_id,
            permissions.clone(),
        );

        let token = create_jwt(claims).map_err(|e| ErrorBadRequest(e))?;

        Ok(HttpResponse::Ok().json(AccessTokenResponse {
            token,
            permissions,
            expiry: config.expiry,
        }))
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid credentials."))
    }
}
