use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Result};
use std::path::Path;

#[get("/")]
async fn login() -> Result<NamedFile> {
    let path = Path::new("static").join("login").join("login.html");

    Ok(NamedFile::open(path)?)
}
