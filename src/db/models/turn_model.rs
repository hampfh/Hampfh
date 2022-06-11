use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::Turns;
use crate::db::schema::Turns::dsl::Turns as turns_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "Turns"]
pub struct Turn {
    pub id: String,
	pub match_id: String,
    pub turn: i32,
	pub board: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Turn {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        turns_dsl.load::<Turn>(conn).expect("Error loading turns")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = turns_dsl.find(id).get_result::<Turn>(conn) {
            Some(record)
        } else {
            None
        }
    }
    
    pub fn create(match_id: &str, turn: i32, board: &str, conn: &SqliteConnection) -> Option<Self> {        
		let new_id = Uuid::new_v4().to_hyphenated().to_string();

        // Make sure match exists
        let match_ = Turn::by_id(&match_id, conn);
        if match_.is_none() {
            return None;
        }

        let new_match = Self::new_turn_struct(&new_id, match_id, turn, board);

        diesel::insert_into(turns_dsl)
            .values(&new_match)
            .execute(conn)
            .expect("Error saving new turns");
		Self::by_id(&new_id, conn)
    }
    fn new_turn_struct(id: &str, match_id: &str, turn: i32, board: &str) -> Self {
        Turn {
            id: id.into(),
			match_id: match_id.into(),
            turn: turn,
            board: board.into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}