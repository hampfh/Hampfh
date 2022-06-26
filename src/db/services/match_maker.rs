use crate::db::models::match_model::Match;
use crate::db::models::submission_model::Submission;
use crate::game::entry_point::initialize_game_session;
use crate::game::game::GameState;
use diesel::SqliteConnection;

pub fn match_make(challenger: &Submission, conn: &SqliteConnection) -> Vec<String> {
    // Order by score and pick the submission with the higest score
    let submissions = Submission::list(conn);
    let mut matches = make_selection(submissions);

    let mut new_challenger = challenger.clone();

    let mut errors: Vec<String> = vec![];

    // Match-maker goes here
    for i in 0..matches.len() {
        let result = initialize_game_session(&challenger.script, &matches[i].script);
        let winner: Option<String>;
        let loser: Option<String>;
        match result {
            GameState::PlayerOneWon => {
                new_challenger.score += 1;
                winner = Some(challenger.id.clone());
                loser = Some(matches[i].id.clone());
            }
            GameState::PlayerTwoWon => {
                matches[i].score += 1;
                winner = Some(matches[i].id.clone());
                loser = Some(challenger.id.clone());
            }
            GameState::Running => {
                errors.push(format!("Somehow the game is still running match..."));
                return errors;
            }
            GameState::Error(err) => {
                errors.push(format!("{:?}", err));
                return errors;
            }
        }

        matches[i].save(conn);
        Match::create(&winner.unwrap(), &loser.unwrap(), conn);
    }

    new_challenger.save(conn);

    return errors;
}

fn make_selection(submissions: Vec<Submission>) -> Vec<Submission> {
    let mut submissions = submissions;
    let mut match_queue: Vec<Submission> = Vec::new();

    // Remove all submissions of disqualified bots
    submissions = submissions
        .into_iter()
        .filter(|submission| submission.disqualified == 0)
        .collect();
    // Sort from lowest to highest
    submissions.sort_by(|a, b| a.score.cmp(&b.score));

    if submissions.len() < 10 {
        return submissions;
    }

    // Only pick out then submissions, equally spread
    for i in 0..10 {
        let index = i * submissions.len() / 10;
        match_queue.push(submissions[index].clone());
    }

    return match_queue;
}
