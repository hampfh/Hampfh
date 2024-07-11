use std::process;

use crate::{backend, match_maker::scheduler::run_scheduled_matchmaking};

pub(crate) fn run_match_make() {
    let db_connection = backend::db::establish_connection();
    let conn = match db_connection.get() {
        Ok(conn) => conn,
        Err(error) => {
            println!("Could not establish connection to database: {}", error);
            process::exit(1);
        }
    };

    run_scheduled_matchmaking(&conn);
    process::exit(0);
}
