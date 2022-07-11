use crate::backend::schema::Submissions::dsl::Submissions as submission_dsl;
use crate::{backend::schema::Submissions, external_related::repo_updater::is_plagiarism_enabled};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "Submissions"]
pub struct Submission {
    pub id: String,
    pub user: String,
    pub script: String,
    pub comment: Option<String>,
    pub score: i32,
    pub issue_url: String,
    pub issue_number: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub disqualified: i32, // Boolean SQLite doesn't support booleans
}
impl Submission {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        submission_dsl
            .load::<Submission>(conn)
            .expect("Error loading submissions")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = submission_dsl.find(id).get_result::<Submission>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn by_script(script_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use crate::backend::schema::Submissions::dsl::script;
        if let Ok(record) = submission_dsl
            .filter(script.eq(script_str))
            .first::<Submission>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_score(score_value: i32, conn: &SqliteConnection) -> Option<Self> {
        use crate::backend::schema::Submissions::dsl::score;
        if let Ok(record) = submission_dsl
            .filter(score.eq(score_value))
            .order(Submissions::created_at.asc())
            .first::<Submission>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(
        user_id: &str,
        script: &str,
        comment: Option<&str>,
        score: i32,
        issue_url: &str,
        issue_number: i32,
        conn: &SqliteConnection,
    ) -> Result<Option<Self>, ()> {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();

        // Check that script doens't already exist
        // we do not allow duplicates
        if is_plagiarism_enabled() && Self::by_script(&script, conn).is_some() {
            return Err(());
        }

        let new_submission = Self::new_submission_struct(
            &new_id,
            user_id,
            script,
            comment,
            score,
            issue_url,
            issue_number,
        );

        diesel::insert_into(submission_dsl)
            .values(&new_submission)
            .execute(conn)
            .expect("Error saving new submission");
        Ok(Self::by_id(&new_id, conn))
    }

    pub fn save(&self, conn: &SqliteConnection) {
        use crate::backend::schema::Submissions::dsl::{disqualified, id, score, updated_at};
        diesel::update(submission_dsl.filter(id.eq(&self.id)))
            .set((
                score.eq(self.score),
                updated_at.eq(chrono::Local::now().naive_local()),
                disqualified.eq(self.disqualified),
            ))
            .execute(conn)
            .expect("Could not update record");
    }

    fn new_submission_struct(
        id: &str,
        user_id: &str,
        script: &str,
        comment: Option<&str>,
        score: i32,
        issue_url: &str,
        issue_number: i32,
    ) -> Self {
        Submission {
            id: id.into(),
            user: user_id.into(),
            script: script.into(),
            comment: comment.map(|c| c.into()),
            score: score,
            disqualified: 0,
            issue_url: issue_url.into(),
            issue_number: issue_number,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}