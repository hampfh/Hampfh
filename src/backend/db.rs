use std::path::Path;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::embedded_migrations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub(crate) fn establish_connection() -> DbPool {
    // If no database exists we create it.
    let path_string =
        std::env::var("DATABASE_URL").expect("NO DATABASE_URL specified in .env file");
    let path = Path::new(&path_string);

    if !path.exists() {
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        std::fs::File::create(&path_string).unwrap();
    }

    // set up database connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(path_string);
    let conn = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    return conn;
}

pub(crate) fn run_migrations(conn: &SqliteConnection) {
    embedded_migrations::run_with_output(conn, &mut std::io::stdout())
        .expect("Failed to run migrations");
}
