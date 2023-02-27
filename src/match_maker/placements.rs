use crate::backend::models::submission_model::Submission;
use diesel::SqliteConnection;

use super::match_executor::{execute_match_queue, MatchReport};

pub(crate) fn run_placements(
    challenger: &Submission,
    conn: &SqliteConnection,
) -> Vec<(MatchReport, MatchReport)> {
    // Order by score and pick the submission with the higest score
    let submissions = Submission::list(conn);
    let matches = make_selection(submissions, challenger);

    return execute_match_queue(
        conn,
        matches
            .iter()
            .map(|current| (challenger.clone(), current.clone()))
            .collect(),
    );
}

fn make_selection(submissions: Vec<Submission>, challenger: &Submission) -> Vec<Submission> {
    let mut submissions = submissions;
    let mut match_queue: Vec<Submission> = Vec::new();

    // Filter submissions: We don't allow the challenger to play against: itself, disqualified bots, or bots that are by the same author
    submissions = submissions
        .into_iter()
        .filter(|submission| {
            submission.disqualified == 0
                && submission.id != challenger.id
                && submission.user != challenger.user
        })
        .collect();
    // Sort from lowest to highest
    submissions.sort_by(|a, b| a.wins.cmp(&b.wins));

    if submissions.len() < 10 {
        return submissions;
    }

    // Only pick out ten submissions, equally spread
    for i in 0..10 {
        let index = i * submissions.len() / 10;
        match_queue.push(submissions[index].clone());
    }

    return match_queue;
}
