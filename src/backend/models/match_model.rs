use crate::backend::schema::Matches;
use crate::backend::schema::Matches::dsl::Matches as matches_dsl;
use crate::backend::schema::Turns::dsl::Turns as turns_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{submission_model::Submission, turn_model::Turn, user_model::User};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "Matches"]
pub struct Match {
    pub id: String,
    pub winner: String,
    pub loser: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Match {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        matches_dsl
            .load::<Match>(conn)
            .expect("Error loading matches")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = matches_dsl.find(id).get_result::<Match>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn players(
        &self,
        conn: &SqliteConnection,
    ) -> Option<((User, Submission), (User, Submission))> {
        return Match::get_players(&self.id, conn);
    }

    pub fn get_players(
        id: &str,
        conn: &SqliteConnection,
    ) -> Option<((User, Submission), (User, Submission))> {
        let target_match = match Self::by_id(id, conn) {
            Some(result) => result,
            None => return None,
        };
        let winning_submission = Submission::by_id(&target_match.winner, &conn);
        let losing_sunmission = Submission::by_id(&target_match.loser, &conn);
        if winning_submission.is_none() || losing_sunmission.is_none() {
            return None;
        }

        let winner = User::by_id(&winning_submission.clone().unwrap().user, &conn);
        let loser = User::by_id(&losing_sunmission.clone().unwrap().user, &conn);
        if winner.is_none() || loser.is_none() {
            return None;
        }
        return Some((
            (winner.unwrap(), winning_submission.unwrap()),
            (loser.unwrap(), losing_sunmission.unwrap()),
        ));
    }

    pub fn get_turns(target_match_id: &str, conn: &SqliteConnection) -> Option<Vec<Turn>> {
        use crate::backend::schema::Turns::dsl::match_id;
        match turns_dsl
            .filter(match_id.eq(target_match_id))
            .load::<Turn>(conn)
        {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }

    pub fn create(winner_id: &str, loser_id: &str, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();

        // Make sure both submissions exist
        let winner = Submission::by_id(&winner_id, conn);
        let loser = Submission::by_id(&loser_id, conn);
        if winner.is_none() || loser.is_none() {
            return None;
        }

        let new_match = Self::new_match_struct(&new_id, winner_id, loser_id);

        diesel::insert_into(matches_dsl)
            .values(&new_match)
            .execute(conn)
            .expect("Error saving new matches");
        Self::by_id(&new_id, conn)
    }
    fn new_match_struct(id: &str, winner: &str, loser: &str) -> Self {
        Match {
            id: id.into(),
            winner: winner.into(),
            loser: loser.into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
