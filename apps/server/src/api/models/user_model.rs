use crate::api::schema::Users;
use crate::api::schema::Users::dsl::Users as user_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "Users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl User {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        user_dsl.load::<User>(conn).expect("Error loading users")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = user_dsl.find(id).get_result::<User>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn by_username(username_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use crate::api::schema::Users::dsl::username;
        if let Ok(record) = user_dsl
            .filter(username.eq(username_str))
            .first::<User>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(username: &str, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();

        // Check that user doens't already exist
        if Self::by_username(&username, conn).is_some() {
            return None;
        }

        let new_user = Self::new_user_struct(&new_id, username);

        diesel::insert_into(user_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");
        Self::by_id(&new_id, conn)
    }
    fn new_user_struct(id: &str, username: &str) -> Self {
        User {
            id: id.into(),
            username: username.to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
