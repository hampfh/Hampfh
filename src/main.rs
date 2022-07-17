use actix_web::{web::Data, App, HttpServer};

use crate::backend::{db::run_migrations, services::routes::routes};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

mod backend;
mod cli;
mod external_related;
mod game;
mod match_maker;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().expect("Could not load .env file");
    migrations();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        cli::cli(args);
        return Ok(());
    }

    let port = 8095;

    println!("Listening on port {}", port);
    HttpServer::new(move || {
        let db_connection = backend::db::establish_connection();
        App::new()
            .app_data(Data::new(db_connection))
            .configure(routes)
    })
    .bind((std::env::var("IP").unwrap(), port))?
    .workers(2)
    .run()
    .await
}

fn migrations() {
    let conn = backend::db::establish_connection().get().unwrap();
    run_migrations(&conn);
}
