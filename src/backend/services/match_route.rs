use actix_web::{
    get,
    web::{self, Json},
};
use serde::Serialize;

use crate::backend::{
    self,
    models::{
        match_model::Match, submission_model::Submission, turn_model::Turn, user_model::User,
    },
};

#[derive(Serialize)]
pub(crate) struct HttpMatch {
    id: String,
    winner: User,
    winner_submission: Submission,
    loser: User,
    loser_submission: Submission,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    p1_is_winner: i32,
    match_error: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct HttpResponseStruct {
    result: HttpMatch,
    turns: Vec<Turn>,
}

#[get("/api/matches/{id}")]
pub(super) async fn get_match_route(
    path: web::Path<String>,
) -> actix_web::Result<Json<HttpResponseStruct>> {
    let conn = backend::db::establish_connection().get().unwrap();
    let id = path.into_inner();

    let target_match = if let Some(target_match) = Match::by_id(&id, &conn) {
        target_match
    } else {
        return Err(actix_web::error::ErrorNotFound(
            "Could not find your match...",
        ));
    };

    let turns = if let Some(turns) = Turn::by_match_id(&id, &conn) {
        turns
    } else {
        return Err(actix_web::error::ErrorNotFound(
            "There are no turns for this match...",
        ));
    };

    let (winner, loser) = if let Some(result) = Match::get_players(&target_match.id, &conn) {
        result
    } else {
        return Err(actix_web::error::ErrorNotFound(
            "There are no players associated with this match...",
        ));
    };

    return Ok(Json(HttpResponseStruct {
        result: HttpMatch {
            id: target_match.id,
            winner: winner.0,
            winner_submission: winner.1,
            loser: loser.0,
            loser_submission: loser.1,
            created_at: target_match.created_at,
            updated_at: target_match.updated_at,
            p1_is_winner: target_match.p1_is_winner,
            match_error: target_match.match_error,
        },
        turns,
    }));
}
