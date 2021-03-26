pub fn with_connection<R, F: FnOnce(&mut Connection) -> R>(data: &Data<Backend>, f: F) -> R {
    let conn = data.conn.clone();
    let conn = &mut conn.lock().unwrap();
    f(conn)
}
