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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::cli(args);
}
