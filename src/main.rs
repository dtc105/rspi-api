mod config;
mod routes;
mod models;

use actix_web::{App, HttpServer, dev::Server, middleware::Logger, web};
use config::{database, dotenv, cors};
use routes::handlers::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::init();
    println!("Environment variables loaded! 🔃");
    
    let db: web::Data<database::AppState> = database::init();
    println!("Connected to database! 💾");

    let port: u16 = std::env::var("PORT")
        .expect("PORT must be defined in `.env`.")
        .parse()
        .unwrap();
    
    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(health_checker_handler)
            .wrap(cors::options())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run();

    println!("Server running on port {port}! 🚀");

    return server.await;
}
