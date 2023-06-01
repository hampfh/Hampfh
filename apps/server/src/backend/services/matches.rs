use actix_web::{get, web::Json};
use serde::Serialize;

use crate::backend::{self, models::match_model::Match};

#[derive(Serialize)]
pub(crate) struct HttpResponseStruct {
    matches: Vec<String>,
}

#[get("/api/matches")]
pub(super) async fn get_matches_route() -> actix_web::Result<Json<HttpResponseStruct>> {
    let conn = backend::db::establish_connection().get().unwrap();

    let matches = Match::list_ids(&conn);

    return Ok(Json(HttpResponseStruct { matches }));
}
