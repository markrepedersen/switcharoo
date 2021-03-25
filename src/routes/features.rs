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

use crate::Backend;

#[derive(Serialize, Deserialize, Debug)]
pub struct KV {
    pub key: String,
    pub val: bool,
}

impl KV {
    pub fn new(key: String, val: bool) -> Self {
        Self { key, val }
    }
}

#[get("/{feature}")]
pub async fn is_toggled(Path(feature): Path<String>, data: Data<Backend>) -> HttpResponse {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();

    match conn.get(&feature).unwrap() {
        Some(val) => HttpResponse::Ok().json(KV::new(feature, val)),
        None => HttpResponse::NotFound().json(format!("Key {} wasn't found.", feature)),
    }
}

#[post("")]
pub async fn set_toggle(payload: Json<KV>, data: Data<Backend>) -> HttpResponse {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();
    let _: () = conn.set(payload.key.clone(), payload.val).unwrap();

    HttpResponse::Ok().finish()
}

#[delete("/{feature}")]
pub async fn remove_toggle(Path(feature): Path<String>, data: Data<Backend>) -> HttpResponse {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();
    let _: () = conn.del(&feature).unwrap();

    HttpResponse::Ok().finish()
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
        if with_connection(&data, |conn| conn.set::<String, bool, ()>(key.clone(), val)).is_err() {
            return Ok(HttpResponse::BadRequest().body(format!("Unable to set ({}, {})", key, val)));
        }
    }

    Ok(HttpResponse::Created().finish())
}

fn with_connection<R, F: FnOnce(&mut Connection) -> R>(data: &Data<Backend>, f: F) -> R {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();
    f(conn)
}
