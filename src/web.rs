use actix_web::{get, HttpResponse};

const INDEX_HTML_FILE: &'static str = include_str!("../web/dist/index.html");
const BUNDLE_JS_FILE: &'static str = include_str!("../web/dist/bundle.js");

#[get("")]
pub fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_HTML_FILE)
}

#[get("/bundle.js")]
pub fn bundle() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(BUNDLE_JS_FILE)
}
