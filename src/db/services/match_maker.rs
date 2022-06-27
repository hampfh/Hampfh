use crate::db::models::match_model::Match;
use crate::db::models::submission_model::Submission;
use crate::db::models::turn_model::Turn;
use crate::game::board::board_to_string;
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
        let (result, turns) = initialize_game_session(&challenger.script, &matches[i].script);
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

        // We never save a match if it wasn't successful
        // If we get thos this point we know there
        // were no errors

        // If there are errors, then we stop the match-making process
        // This is because the submitted bot is obviously not working
        // and should therefore not be matchmaked against future bots

        matches[i].save(conn);
        match Match::create(&winner.unwrap(), &loser.unwrap(), conn) {
            Some(match_record) => {
                // Generate turns
                let mut turn_index = 1;
                for turn in turns {
                    Turn::create(&match_record.id, turn_index, &board_to_string(turn), conn);
                    turn_index += 1;
                }
            }
            None => {
                errors.push(format!(
                    "Internal error, match making interrupted, try again later"
                ));
                return errors;
            }
        }
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
