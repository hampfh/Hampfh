use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::get;

#[get("/")]
pub(super) async fn get_ping() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    return Ok(NamedFile::open(path).unwrap());
}

#[get("/api/ping")]
pub(super) async fn get_api_ping() -> actix_web::Result<String> {
    return Ok("pong".to_string());
}
