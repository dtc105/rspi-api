use actix_web::web::Data;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct AppState {
    db: Mutex<Connection>,
}

pub fn init() -> Data<AppState> {
    let db_path: String = std::env::var("DB_PATH").expect("DB_PATH in .env must be set");
    let db_conn: Connection = Connection::open(db_path).expect("Couldn't connect to database!");

    Data::new(AppState { db: Mutex::new(db_conn) })
}