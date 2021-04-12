use actix_web::{
    post,
    delete,
    error::ErrorInternalServerError,
    get, put,
    web::ServiceConfig,
    web::{scope, Data, Json, Path},
    HttpResponse, Result,
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use auth::claim::decode_jwt;
use serde::{Deserialize, Serialize};

use crate::{auth, backends::RedisBackend, validate_jwt};

type HttpResult = Result<HttpResponse>;

#[derive(Serialize, Deserialize)]
pub struct FeatureFlag {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub key: String,
    pub value: bool,
}

impl FeatureFlag {
    pub fn new(key: String, value: bool) -> Self {
        Self { key, value }
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/features")
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .service(get_flag)
            .service(update_flag)
            .service(remove_flag)
            .service(all_flags)
            .service(add_flag),
    );
}

#[get("/{key}")]
pub async fn get_flag(key: Path<String>, data: Data<RedisBackend>, auth: BearerAuth) -> HttpResult {
    let key = key.into_inner();
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;
    let val = data
        .get_flag(key.clone(), tenant_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(FeatureFlag::new(key, val)))
}


#[put("/{key}")]
pub async fn update_flag(
    key: Path<String>,
    payload: Json<FeatureFlag>,
    data: Data<RedisBackend>,
    auth: BearerAuth,
) -> HttpResult {
    let key = key.into_inner();
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;

    data.add_or_update_flag(payload.key.clone(), payload.value, tenant_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(FeatureFlag::new(key, payload.value)))
}

#[delete("/{key}")]
pub async fn remove_flag(
    key: Path<String>,
    data: Data<RedisBackend>,
    auth: BearerAuth,
) -> HttpResult {
    let key = key.into_inner();
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;

    data.remove_flag(key.clone(), tenant_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(FeatureFlag::new(key, false)))
}

#[get("")]
pub async fn all_flags(data: Data<RedisBackend>, auth: BearerAuth) -> HttpResult {
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;
    let flags: Vec<FeatureFlag> = data
        .get_all_flags(tenant_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?
        .iter()
        .map(|(key, val)| FeatureFlag::new(key.clone(), val.clone()))
        .collect();

    Ok(HttpResponse::Ok().json(flags))
}

#[post("")]
pub async fn add_flag(
    payload: Json<FeatureFlag>,
    data: Data<RedisBackend>,
    auth: BearerAuth,
) -> HttpResult {
    let claims = decode_jwt(auth.token())?;
    let tenant_id = claims.tenant_id;

    data.add_or_update_flag(payload.key.clone(), payload.value, tenant_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(FeatureFlag::new(payload.key.clone(), payload.value)))
}
