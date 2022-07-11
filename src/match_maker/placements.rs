use crate::backend::models::match_model::Match;
use crate::backend::models::submission_model::Submission;
use crate::backend::models::turn_model::Turn;
use crate::game::board::board_to_string;
use crate::game::entry_point::initialize_game_session;
use crate::game::game::{ErrorType, GameResult};
use crate::game::player::PlayerType;
use diesel::SqliteConnection;

pub fn run_placements(challenger: &Submission, conn: &SqliteConnection) -> Vec<String> {
    // Order by score and pick the submission with the higest score
    let submissions = Submission::list(conn);
    let mut matches = make_selection(submissions, challenger.id.clone());

    let mut new_challenger = challenger.clone();

    let mut round_reports: Vec<String> = Vec::new();

    // Match-maker goes here
    for i in 0..matches.len() {
        let mut error_fault: Option<PlayerType> = None;
        let mut error_msg: Option<String> = None;

        let (result, turns) = initialize_game_session(&challenger.script, &matches[i].script);
        let winner: Option<String>;
        let loser: Option<String>;
        match result {
            GameResult::PlayerOneWon => {
                new_challenger.score += 1;
                winner = Some(challenger.id.clone());
                loser = Some(matches[i].id.clone());
            }
            GameResult::PlayerTwoWon => {
                matches[i].score += 1;
                winner = Some(matches[i].id.clone());
                loser = Some(challenger.id.clone());
            }
            GameResult::Error(error) => {
                // If an error occur, it's no longer a matter
                // of who is the winner, it's rather a matter
                // of who is the going to be disqualified.
                winner = None;
                loser = None;

                match error {
                    ErrorType::GameError { reason, fault }
                    | ErrorType::RuntimeError { reason, fault } => {
                        error_fault = fault;
                        error_msg = Some(reason);
                        matches[i].disqualified = 1;
                    }
                    ErrorType::TurnTimeout { fault } => {
                        error_fault = fault;
                        error_msg = Some("Turn timeout".to_string());
                        matches[i].disqualified = 1;
                    }
                    ErrorType::GameDeadlock => {
                        error_msg = Some("Deadlock, both bots failed".to_string());
                        matches[i].disqualified = 1;
                    }
                }

                // Challenger is always the flipped player
                match error_fault {
                    Some(PlayerType::Regular) => {
                        println!("Disq reg");
                        matches[i].disqualified = 1;
                    }
                    Some(PlayerType::Flipped) => {
                        println!("Disq flip");
                        new_challenger.disqualified = 1;
                    }
                    None => {
                        println!("Disq both");
                        // Both are disqualified
                        matches[i].disqualified = 1;
                        new_challenger.disqualified = 1;
                    }
                }
            }
        }

        round_reports.push(report_round(
            error_msg.clone(),
            error_fault.clone(),
            matches[i].id.clone(),
        ));

        matches[i].save(conn);

        // If the new challenger has a part in the error
        // we disqualify it directly here
        if error_msg.is_some()
            && error_fault.is_some()
            && error_fault.unwrap() == PlayerType::Flipped
            || winner.is_none()
            || loser.is_none()
        {
            println!("Saving {:?}", new_challenger);
            new_challenger.save(conn);
            return round_reports;
        }

        // We never save a match if it wasn't successful
        // If we get thos this point we know there
        // were no errors

        // If there are errors, then we stop the match-making process
        // This is because the submitted bot is obviously not working
        // and should therefore not be matchmaked against future bots
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
                println!("Internal error, could not create match");
            }
        }
    }

    new_challenger.save(conn);
    return round_reports;
}

fn report_round(
    error_msg: Option<String>,
    fault: Option<PlayerType>,
    opponent_id: String,
) -> String {
    match error_msg {
        Some(error_msg) => format!(
            "[FAIL] Opponent: {}, Error: {}, {}",
            opponent_id,
            error_msg,
            match fault {
                Some(PlayerType::Flipped) => format!("submission has been disqualified"),
                Some(PlayerType::Regular) => format!("opponent has been disqualified"),
                None => format!("both players have been disqualifed"),
            }
        ),
        None => format!("[SUCCESS] Opponent: {}", opponent_id),
    }
}

fn make_selection(submissions: Vec<Submission>, challenger_id: String) -> Vec<Submission> {
    let mut submissions = submissions;
    let mut match_queue: Vec<Submission> = Vec::new();

    // Remove all submissions of disqualified bots
    submissions = submissions
        .into_iter()
        .filter(|submission| submission.disqualified == 0 && submission.id != challenger_id)
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
