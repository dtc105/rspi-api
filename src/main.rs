mod routes;
mod models;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, get, http::header, middleware::Logger, web};
use dotenv::dotenv;
use routes::health_checker::health_checker_handler;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct AppState {
    db: Mutex<Connection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();

    let db_path: String = std::env::var("DB_PATH").expect("DB_PATH in .env must be set");
    let db_conn: Connection = Connection::open(db_path).expect("Couldn't connect to database!");

    let shared_state = web::Data::new(AppState {
        db: Mutex::new(db_conn),
    });

    println!("Connected to database! 💾");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(shared_state.clone()))
            .service(health_checker_handler)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 5174))?
    .run()
    .await
}
