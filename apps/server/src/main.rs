use clap::{arg, command, value_parser, Arg, Command};

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
pub(crate) async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().expect("Could not load .env file");

    let matches = command!()
        .arg_required_else_help(true)
        .version("1.1.0")
        .author("Hampus Hallkvist <hampus@hallkvist.org>")
        .about("Bot battle runtime - A game, server and debugger baked into one. This binary can run matches, host a webserver connecting to github, or interact with a database.\nAuthored by: Hampus Hallkvist @hampfh")
        .subcommand(
            Command::new("match")
                .about("Run a match between two bots")
                .long_about("Specify two paths to lua scripts to run a local match between them.")
                .arg(Arg::new("path-one").value_name("PATH").help("Path to first bot (.lua)").required(true))
                .arg(Arg::new("path-two").value_name("PATH").help("Path to second bot (.lua)").required(true))
                .arg(arg!(--gif "Generate gif of match").num_args(0))
                .arg(arg!(--readme "Generate readme of match").num_args(0))
                .arg(arg!(--print "Print the match in console").num_args(0))
                .arg(arg!(--log "Generate a move log, file containing all moves").num_args(0))
                .arg(arg!(-o --output "Output directory for files").default_value("output").value_parser(value_parser!(String)),)
                .arg(Arg::new("init-timeout").value_name("TIMEOUT").help("Time in milliseconds that the script has to initialize").default_value("250").value_parser(value_parser!(u64)))
                .arg(Arg::new("turn-timeout").value_name("TIMEOUT").help("Time in milliseconds that the script has to return a move").default_value("250").value_parser(value_parser!(u64)))
        )
        .subcommand(
            Command::new("db")
                .about("Database interaction commands")
                .subcommand(
                    Command::new("generate")
                        .about("Generate match files and history").long_about("Fetch all matches from database and generate files in a github friendly format")
                        .arg(arg!(-p --primary "Generate primary README file"))
                        .arg(arg!(-m --matches "Generate match files"))
                        .arg(arg!(-l --logs "Generate match log files"))
                        .arg(arg!(-c --clear "Clear match directory")),
                ),
        )
        .subcommand(
            Command::new("server")
                .about("Start http server")
                .arg(
                    arg!(
                        --host <HOST> "ip address to listen on"
                    )
                    .required(false)
                    .default_value("127.0.0.1")
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(--port <PORT> "Port to listen on")
                        .required(false)
                        .default_value("8000")
                        .value_parser(value_parser!(u16)),
                ),
        )
        .subcommand(
            Command::new("matchmake")
                .about("Start matchmaker")
        )
        .get_matches();

    cli::cli::cli(matches).await?;

    Ok(())
}
