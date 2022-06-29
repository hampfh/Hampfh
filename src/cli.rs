use actix_web::{web::Data, App, HttpServer};

use crate::{
    db::{
        self,
        models::{
            match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
        },
        services::core::config,
    },
    readme_factory::{build_match_files_wrapper, clear_match_dir, generate_readme, write_file},
};

pub fn cli(args: Vec<String>) {
    match args[1].as_str() {
        "generate-main" => generate_main(),
        "generate-matches" => build_match_files_wrapper(),
        "clear" => clear_match_dir(),
        "server" => server().unwrap(),
        _ => {}
    }
}

fn generate_main() {
    let conn = db::db::establish_connection().get().unwrap();
    write_file(
        "README.md",
        generate_readme(
            User::list(&conn),
            Submission::list(&conn),
            Match::list(&conn),
            Turn::list(&conn),
        ),
    )
    .unwrap();
}

#[actix_web::main]
async fn server() -> Result<(), std::io::Error> {
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
