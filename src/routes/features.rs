use actix_multipart::Multipart;
use actix_web::{
    delete, get, post,
    web::{Buf, Data, Json, Path},
    Error, HttpResponse,
};
use futures::{StreamExt, TryStreamExt};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::collections::HashMap;

use crate::{backends::redis::with_connection, Backend};

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
pub async fn is_toggled(Path(feature): Path<String>, data: Data<Backend>) -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        match conn.get(&feature).unwrap() {
            Some(val) => HttpResponse::Ok().json(KV::new(feature, val)),
            None => HttpResponse::NotFound().json(format!("Key {} wasn't found.", feature)),
        }
    })
}

#[delete("/{feature}")]
pub async fn remove_toggle(Path(feature): Path<String>, data: Data<Backend>) -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        conn.del::<String, bool>(feature).unwrap();
        HttpResponse::Ok().finish()
    })
}

#[get("")]
pub async fn all_toggles(data: Data<Backend>) -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        let keys: Vec<String> = match conn.scan::<String>() {
            Ok(keys) => keys.collect(),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let values: Vec<String> = conn.get(keys.clone()).unwrap();

        if keys.len() != values.len() {
            HttpResponse::InternalServerError()
                .body("Unable to retrieve values for all keys; lengths do not match.")
        } else {
            let pairs: Vec<KV> = keys
                .iter()
                .enumerate()
                .map(|(i, key)| KV::new(key.clone(), values[i].parse().unwrap_or(false)))
                .collect();

            HttpResponse::Ok().json(pairs)
        }
    })
}

#[post("")]
pub async fn set_toggle(payload: Json<KV>, data: Data<Backend>) -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        conn.set::<String, bool, ()>(payload.key.clone(), payload.value)
            .unwrap();
        HttpResponse::Ok().finish()
    })
}

#[post("/import")]
pub async fn import_toggles(
    mut payload: Multipart,
    data: Data<Backend>,
) -> Result<HttpResponse, Error> {
    let mut buf: Vec<u8> = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        while let Some(chunk) = field.next().await {
            buf.extend_from_slice(chunk?.bytes());
        }
    }

    for (key, val) in from_slice::<HashMap<String, bool>>(&mut buf)? {
        let result = with_connection(&data, |conn: &mut Connection| {
            conn.set::<String, bool, ()>(key.clone(), val)
        });

        if result.is_err() {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Unable to set ({}, {})", key, val)));
        }
    }

    Ok(HttpResponse::Created().finish())
}
