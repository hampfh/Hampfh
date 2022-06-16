use actix_web::web::Data;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod game;
mod readme_factory;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use crate::db::services::core::config;
    
    let port = 8095;

    println!("Listening on port {}", port);
    HttpServer::new(|| {
        let db_connection = db::db::establish_connection();

        App::new()
            .app_data(Data::new(db_connection))
            .configure(config)
    })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}