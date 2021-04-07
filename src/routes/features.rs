use actix_multipart::Multipart;
use actix_web::{
    delete,
    error::ErrorInternalServerError,
    get, post, put,
    web::ServiceConfig,
    web::{scope, Buf, Data, Json, Path},
    HttpResponse, Result,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use futures::{stream::StreamExt, TryStreamExt};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::collections::HashMap;

use crate::{validate_jwt, Backend};

type HttpResult = Result<HttpResponse>;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/features")
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .service(get_toggle)
            .service(all_toggles)
            .service(update_toggle)
            .service(remove_toggle)
            .service(import_toggles),
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub id: String,
    pub value: bool,
}

impl Feature {
    pub fn new(key: String, value: bool) -> Self {
        Self { id: key, value }
    }

    pub fn normalize<T: Into<bool>>(val: T) -> String {
        let b: bool = val.into();

        if b {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

#[get("/{feature}")]
pub async fn get_toggle(Path(feature): Path<String>, data: Data<Backend>) -> HttpResult {
    Ok(match data.conn.lock() {
        Ok(mut conn) => {
            let val: String = conn
                .get(&feature)
                .await
                .map_err(|e| ErrorInternalServerError(e))?;

            HttpResponse::Ok().json(Feature::new(feature, val.parse::<bool>().expect("Expected boolean String.")))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[put("/{feature}")]
pub async fn update_toggle(
    Path(feature): Path<String>,
    payload: Json<Feature>,
    data: Data<Backend>,
) -> HttpResult {
    Ok(match data.conn.lock() {
        Ok(mut conn) => {
            conn
                .set(&feature, Feature::normalize(payload.value))
                .await
                .map_err(|e| ErrorInternalServerError(e))?;

            HttpResponse::Ok().json(feature)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[delete("/{feature}")]
pub async fn remove_toggle(Path(feature): Path<String>, data: Data<Backend>) -> HttpResult {
    Ok(match data.conn.lock() {
        Ok(mut conn) => {
            conn
                .del(&feature)
                .await
                .map_err(|e| ErrorInternalServerError(e))?;

            HttpResponse::Ok().json(feature)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[get("")]
pub async fn all_toggles(data: Data<Backend>) -> HttpResult {
    Ok(match data.conn.lock() {
        Ok(mut conn) => {
            let mut iter = conn.scan().await.map_err(|e| ErrorInternalServerError(e))?;
	    let mut pairs: Vec<Feature> = Vec::new();
	    let mut keys = Vec::new();

            while let Some(key) = iter.next_item().await {
                keys.push(key);
            }

            for key in keys {
                let val: String = conn
                    .get(&key)
                    .await
                    .map_err(|e| ErrorInternalServerError(e))?;

                pairs.push(Feature::new(key, val.parse::<bool>().expect("Expected boolean String.")));
            }

            HttpResponse::Ok().json(pairs)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    })
}

#[post("/import")]
pub async fn import_toggles(mut payload: Multipart, data: Data<Backend>) -> HttpResult {
    let mut buf: Vec<u8> = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        while let Some(chunk) = field.next().await {
            buf.extend_from_slice(chunk?.bytes());
        }
    }

    for (key, val) in from_slice::<HashMap<String, bool>>(&mut buf)? {
        if let Ok(mut conn) = data.conn.lock() {
            conn.set::<String, String, ()>(key.clone(), Feature::normalize(val))
                .await
                .map_err(|e| ErrorInternalServerError(e))?;
        }
    }

    Ok(HttpResponse::Created().finish())
}
