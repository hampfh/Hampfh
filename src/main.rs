use actix_web::{web::Data, App, HttpServer};

use crate::db::services::core::config;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod cli;
mod code_unwrapper;
mod db;
mod game;
mod readme_factory;
mod repo_updater;
mod terminate_thread;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    cli::cli(args);

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
