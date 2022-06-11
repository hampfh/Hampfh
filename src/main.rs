#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod game;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web::JsonConfig};
    use crate::db::services::core::submit_challenge;
    
    let port = 8095;

    HttpServer::new(move || {
        let db_connection = db::db::establish_connection();

        println!("Listening on port {}", port);

        App::new()
            .app_data(db_connection)
            .app_data(JsonConfig::default())
            .service(submit_challenge)
    })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}