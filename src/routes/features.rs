use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::Backend;

#[derive(Serialize, Deserialize)]
pub struct Toggle {
    pub key: String,
    pub val: bool,
}

impl Toggle {
    pub fn new(key: String, val: bool) -> Self {
        Self { key, val }
    }
}

#[get("/{feature}")]
pub async fn is_toggled(Path(feature): Path<String>, data: Data<Backend>) -> HttpResponse {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();
    let val = conn.get(&feature).unwrap();

    HttpResponse::Ok().json(Toggle::new(feature, val))
}

#[post("/")]
pub async fn set_toggle(payload: Json<Toggle>, data: Data<Backend>) -> HttpResponse {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();
    let _: () = conn.set(payload.key.clone(), payload.val).unwrap();
    HttpResponse::Ok().finish()
}
