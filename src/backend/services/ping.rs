use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::get;

#[get("/")]
pub(super) async fn get_ping() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./src/backend/static/index.html".parse().unwrap();
    return Ok(NamedFile::open(path).unwrap());
}
