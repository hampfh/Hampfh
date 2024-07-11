use actix_web::{web::Data, App, HttpServer};

use crate::backend::{self, db::run_migrations, services::routes::routes};

pub(crate) async fn start_server(port: u16, host: String) -> Result<(), std::io::Error> {
    migrations();

    println!("Listening on port {} and host {}", port, host);
    HttpServer::new(move || {
        let db_connection = backend::db::establish_connection();
        App::new()
            .app_data(Data::new(db_connection))
            .configure(routes)
    })
    .bind((host, port))?
    .workers(2)
    .run()
    .await
}

fn migrations() {
    let conn = backend::db::establish_connection().get().unwrap();
    run_migrations(&conn);
}
