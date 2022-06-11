use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

embed_migrations!();

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    
    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    return r2d2::Pool::builder()
       .build(manager)
       .expect("Failed to create pool.");
}