use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::Backend;

#[derive(Serialize, Deserialize)]
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

#[post("/")]
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
