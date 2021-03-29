use actix_web::{get, post, delete, HttpResponse};
use redis::{Commands, Connection};
use redis::with_connection;

#[get("")]
async fn get_users() -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        conn.acl_users();
    })
}

#[post("")]
async fn add_user() -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        conn.set_user();
    })
}

#[delete("/{id}")]
async fn delete_user() -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        conn.del_user();
    })
}

#[get("/{id}")]
async fn get_users_by_id() -> HttpResponse {
    with_connection(&data, |conn: &mut Connection| {
        conn.get_user();
    })
}