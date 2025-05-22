mod config;
mod controllers;
mod dtos;
mod models;
mod routes;

use actix_web::{App, HttpServer, dev::Server, middleware::Logger, web};
use config::{cors, database, dotenv};
use routes::routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::init();

    let db: web::Data<database::AppState> = database::init();

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be defined in `.env`.")
        .parse()
        .unwrap();

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(cors::options())
            .wrap(Logger::default())
            .configure(router)
    })
    .bind(("127.0.0.1", port))?
    .run();

    println!("Server running on port {port}! 🚀");

    server.await
}
