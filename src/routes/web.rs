use actix_web::{body::Body, dev::ResourcePath, web::Path};
use actix_web::{get, HttpResponse};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    let path = format!("dist/{}", path);

    match Asset::get(&path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("File not found"),
    }
}

#[get("/{path:.*}")]
pub fn dist(path: Path<String>) -> HttpResponse {
    if path.is_empty() {
        handle_embedded_file("index.html")
    } else {
        handle_embedded_file(&path.path())
    }
}
