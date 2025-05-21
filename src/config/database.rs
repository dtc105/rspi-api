use actix_web::web::Data;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;

pub struct AppState {
    pub pool: Pool<SqliteConnectionManager>,
}

pub fn init() -> Data<AppState> {
    let db_path: String = std::env::var("DB_PATH").expect("DB_PATH in .env must be set");

    let manager = SqliteConnectionManager::file(db_path).with_flags(
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
    );

    let pool = Pool::new(manager).expect("Failed to created SQLite pool.");

    println!("Connected to database! 💾");

    Data::new(AppState { pool })
}
