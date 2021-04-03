use actix_web::{get, web::ServiceConfig, HttpResponse};

static INDEX_HTML_FILE: &'static str = include_str!("../../web/dist/index.html");
static BUNDLE_JS_FILE: &'static str = include_str!("../../web/dist/bundle.js");

static HTML_CONTENT_TYPE: &'static str = "text/html; charset=utf-8";
static JS_CONTENT_TYPE: &'static str = "text/javascript";

#[get("/")]
pub fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(INDEX_HTML_FILE)
}

#[get("/bundle.js")]
pub fn bundle() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(JS_CONTENT_TYPE)
        .body(BUNDLE_JS_FILE)
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index).service(bundle);
}
