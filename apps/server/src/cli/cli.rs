use crate::backend::{self};
use clap::ArgMatches;

use super::{generate::generate, match_make::run_match_make, r#match::run_local_match};

pub async fn cli(matches: ArgMatches) -> Result<(), std::io::Error> {
    match matches.subcommand() {
        Some(("match", sub_m)) => run_local_match(sub_m),
        Some(("db", sub_m)) => match sub_m.subcommand() {
            Some(("generate", sub_m)) => generate(sub_m),
            _ => return Ok(()),
        },
        Some(("server", sub_m)) => {
            let port = sub_m.get_one::<u16>("port").unwrap();
            let host = sub_m.get_one::<String>("host").unwrap();
            backend::server::start_server(*port, host.clone()).await?;
        }
        Some(("matchmake", _)) => run_match_make(),
        _ => {}
    }
    Ok(())
}
