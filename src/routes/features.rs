use actix_multipart::Multipart;
use actix_web::{
    delete,
    error::ErrorInternalServerError,
    get, post,
    web::ServiceConfig,
    web::{scope, Buf, Data, Json, Path},
    HttpResponse, Result,
};
use futures::{StreamExt, TryStreamExt};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::collections::HashMap;

use crate::{backends::redis::with_connection, Backend};

type HttpResult = Result<HttpResponse>;

#[derive(Serialize, Deserialize, Debug)]
pub struct KV {
    pub key: String,
    pub value: bool,
}

impl KV {
    pub fn new(key: String, value: bool) -> Self {
        Self { key, value }
    }
}

#[get("/{feature}")]
pub async fn is_toggled(Path(feature): Path<String>, data: Data<Backend>) -> HttpResult {
    with_connection(&data, |conn: &mut Connection| -> HttpResult {
        let val = conn
            .get(&feature)
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().json(KV::new(feature, val)))
    })
}

#[delete("/{feature}")]
pub async fn remove_toggle(Path(feature): Path<String>, data: Data<Backend>) -> HttpResult {
    with_connection(&data, |conn: &mut Connection| -> HttpResult {
        conn.del::<String, bool>(feature)
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().finish())
    })
}

#[get("")]
pub async fn all_toggles(data: Data<Backend>) -> HttpResult {
    with_connection(&data, |conn: &mut Connection| -> HttpResult {
        let keys: Vec<String> = conn
            .scan::<String>()
            .map_err(|e| ErrorInternalServerError(e))?
            .collect();

        if keys.is_empty() {
            let values: Vec<String> = conn
                .get(keys.clone())
                .map_err(|e| ErrorInternalServerError(e))?;
            let pairs: Vec<KV> = keys
                .iter()
                .enumerate()
                .map(|(i, key)| KV::new(key.clone(), values[i].parse().unwrap_or(false)))
                .collect();

            Ok(HttpResponse::Ok().json(pairs))
        } else {
            Ok(HttpResponse::Ok().finish())
        }
    })
}

#[post("")]
pub async fn set_toggle(payload: Json<KV>, data: Data<Backend>) -> HttpResult {
    with_connection(&data, |conn: &mut Connection| -> HttpResult {
        conn.set::<String, bool, ()>(payload.key.clone(), payload.value)
            .map_err(|e| ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().finish())
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
        with_connection(
            &data,
            |conn: &mut Connection| -> Result<(), actix_web::Error> {
                conn.set::<String, bool, ()>(key.clone(), val)
                    .map_err(|e| ErrorInternalServerError(e))
            },
        )?;
    }

    Ok(HttpResponse::Created().finish())
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/features")
            .service(is_toggled)
            .service(all_toggles)
            .service(set_toggle)
            .service(remove_toggle)
            .service(import_toggles),
    );
}
