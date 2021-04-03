use actix_session::Session;
use actix_web::{
    get, post, web::Data, web::Json, web::ServiceConfig, HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    user: Json<UserRequest>,
    data: Data<PgPool>,
    session: Session,
) -> impl Responder {
    match User::is_signed_in(&user.into_inner(), data.get_ref()).await {
        Ok(db_user) => match session.set("user_id", db_user.id) {
            Ok(_) => {
                session.renew();

                HttpResponse::Ok().json(db_user)
            }
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        _ => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

#[post("/logout")]
async fn logout(session: Session) -> Result<HttpResponse> {
    if let Some(id) = session.get::<Uuid>("user_id")? {
        session.purge();
        Ok(format!("Logged out: {}", id).into())
    } else {
        Ok("Could not log out anonymous user.".into())
    }
}

#[post("/register")]
pub async fn register(user: Json<UserRequest>, data: Data<PgPool>) -> impl Responder {
    match User::create(user.into_inner(), data.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/whoami")]
pub async fn whoami(session: Session, data: Data<PgPool>) -> impl Responder {
    match session.get::<Uuid>("user_id") {
        Ok(Some(id)) => match User::find_by_id(id, data.as_ref()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
    cfg.service(whoami);
}
