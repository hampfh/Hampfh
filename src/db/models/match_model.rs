use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::Matches;
use crate::db::schema::Matches::dsl::Matches as matches_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
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
        matches_dsl.load::<Match>(conn).expect("Error loading matches")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = matches_dsl.find(id).get_result::<Match>(conn) {
            Some(record)
        } else {
            None
        }
    }
    
    pub fn create(winner_id: &str, loser_id: &str, conn: &SqliteConnection) -> Option<Self> {        
		let new_id = Uuid::new_v4().to_hyphenated().to_string();

		// Make sure both submissions exist
        let winner = Match::by_id(&winner_id, conn);
        let loser = Match::by_id(&loser_id, conn);
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