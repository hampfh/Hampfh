use diesel::SqliteConnection;

use crate::{
    backend::models::{match_model::Match, submission_model::Submission, turn_model::Turn},
    external_related::repo_updater::get_issue_url,
    game::{
        board::{board_to_string, Tile},
        entry_point::initialize_game_session,
        game::{ErrorType, GameResult},
        player::PlayerType,
    },
};

use super::mmr::{calculate_mmr, MMR};

pub(crate) struct MatchReport {
    pub(crate) report: String,
    pub(crate) issue_number: i32,
}

pub(super) fn execute_match_queue(
    conn: &SqliteConnection,
    match_queue: Vec<(Submission, Submission)>,
) -> Vec<(MatchReport, MatchReport)> {
    // TODO if a submission is disqualified and are queued to play multiple
    // TODO games, those games should be skipped.
    let mut round_reports: Vec<(MatchReport, MatchReport)> = Vec::new();
    for i in 0..match_queue.len() {
        let MatchReturn {
            mut p1,
            mut p2,
            winner_id,
            loser_id,
            error_msg,
            error_fault,
            turns,
            critical_error,
        } = start_match(match_queue[i].clone());

        // If the new challenger has a part in the error
        // we disqualify it directly here
        if critical_error
            && error_msg.is_some()
            && error_fault.is_some()
            && error_fault.clone().unwrap() == PlayerType::Flipped
            || winner_id.is_none()
            || loser_id.is_none()
        {
            let report = create_report_text(
                error_msg.clone(),
                error_fault.clone(),
                p1.id.clone(),
                p1.issue_number,
                p2.id.clone(),
                p2.issue_number,
                None,
                None,
                critical_error,
            );

            round_reports.push((
                MatchReport {
                    report: report.0,
                    issue_number: p1.issue_number,
                },
                MatchReport {
                    report: report.1,
                    issue_number: p2.issue_number,
                },
            ));

            p1.save(conn);
            p2.save(conn);
            continue;
        }

        let winner_id = winner_id.unwrap();
        let loser_id = loser_id.unwrap();

        let (p1_new_mmr, p2_new_mmr) = calculate_mmr(
            MMR {
                rating: p1.mmr,
                matches_played: p1.matches_played,
            },
            MMR {
                rating: p2.mmr,
                matches_played: p2.matches_played,
            },
            p1.id == winner_id,
            0.5,
        );

        // Assign new mmr
        p1.mmr = p1_new_mmr;
        p2.mmr = p2_new_mmr;
        // Increment matches played
        p1.matches_played += 1;
        p2.matches_played += 1;

        // TODO create a queue and save all submissions in the end
        // TODO this way if the same submission is playing twice it only needs to be saved once
        p1.save(conn);
        p2.save(conn);

        // We never save a match if it wasn't successful
        // If we get to this point we know there
        // were no errors

        let p1_is_winner = p1.id == winner_id;
        let match_record =
            match Match::create(&winner_id, &loser_id, p1_is_winner, error_msg.clone(), conn) {
                Some(match_record) => {
                    // Generate turns
                    let mut turn_index = 1;
                    for turn in turns {
                        Turn::create(&match_record.id, turn_index, &board_to_string(turn), conn);
                        turn_index += 1;
                    }
                    match_record
                }
                None => {
                    println!("Internal error, could not create match");
                    continue;
                }
            };

        let report = create_report_text(
            error_msg.clone(),
            error_fault.clone(),
            p1.id.clone(),
            p1.issue_number,
            p2.id.clone(),
            p2.issue_number,
            Some(winner_id.clone()),
            Some(format!("../blob/live/data/matches/{}.md", match_record.id)),
            critical_error,
        );
        round_reports.push((
            MatchReport {
                report: report.0,
                issue_number: p1.issue_number,
            },
            MatchReport {
                report: report.1,
                issue_number: p2.issue_number,
            },
        ));
    }
    return round_reports;
}

struct MatchReturn {
    p1: Submission,
    p2: Submission,
    winner_id: Option<String>,
    loser_id: Option<String>,
    turns: Vec<Vec<Tile>>,
    error_msg: Option<String>,
    error_fault: Option<PlayerType>,
    critical_error: bool,
}

fn start_match(players: (Submission, Submission)) -> MatchReturn {
    let mut error_fault: Option<PlayerType> = None;
    let mut error_msg: Option<String> = None;

    let (mut p1, mut p2) = players;

    let (result, turns) = initialize_game_session(&p1.script, &p2.script);
    let mut winner: Option<String> = None;
    let mut loser: Option<String> = None;

    let p1_id = p1.id.clone();
    let p2_id = p2.id.clone();

    let mut critical_error = false;
    match result {
        GameResult::PlayerOneWon => {
            p1.wins += 1;
            winner = Some(p1_id);
            loser = Some(p2_id);
        }
        GameResult::PlayerTwoWon => {
            p2.wins += 1;
            winner = Some(p2_id);
            loser = Some(p1_id);
        }
        GameResult::Error(error) => {
            match error {
                ErrorType::GameError { reason, fault } => {
                    match fault {
                        Some(PlayerType::Regular) => {
                            winner = Some(p1_id);
                            loser = Some(p2_id)
                        }
                        Some(PlayerType::Flipped) => {
                            winner = Some(p2_id);
                            loser = Some(p1_id)
                        }
                        None => (),
                    }
                    error_fault = fault;
                    error_msg = Some(reason);
                }
                ErrorType::RuntimeError { reason, fault } => {
                    error_fault = fault;
                    error_msg = Some(reason);
                    critical_error = true;
                }
                ErrorType::TurnTimeout { fault } => {
                    error_fault = fault;
                    error_msg = Some("Turn timeout".to_string());
                    critical_error = true;
                }
                ErrorType::GameDeadlock => {
                    error_msg = Some("Deadlock, both bots failed".to_string());
                    critical_error = true;
                }
            }

            // Challenger is always the flipped player
            if critical_error {
                match error_fault {
                    Some(PlayerType::Regular) => {
                        println!("Disq reg");
                        p2.disqualified = 1;
                    }
                    Some(PlayerType::Flipped) => {
                        println!("Disq flip");
                        p1.disqualified = 1;
                    }
                    None => {
                        println!("Disq both");
                        // Both are disqualified
                        p1.disqualified = 1;
                        p2.disqualified = 1;
                    }
                }
            }
        }
    }

    return MatchReturn {
        p1,
        p2,
        winner_id: winner,
        loser_id: loser,
        turns,
        error_msg,
        error_fault,
        critical_error,
    };
}

/// Returns two reports, one for p1 and one for p2
fn create_report_text(
    error_msg: Option<String>,
    fault: Option<PlayerType>,
    p1: String,
    p1_issue_number: i32,
    p2: String,
    p2_issue_number: i32,
    winner_id: Option<String>,
    match_url: Option<String>,
    critical_error: bool,
) -> (String, String) {
    let p1_issue = get_issue_url(p1_issue_number);
    let p2_issue = get_issue_url(p2_issue_number);

    match error_msg {
        Some(error_msg) => {
            return (
                // Sent to submission that represents p1
                get_error_report(
                    format!("[{}]({})", p2, p2_issue),
                    error_msg.clone(),
                    match fault.clone() {
                        Some(PlayerType::Regular) => Some(false),
                        Some(PlayerType::Flipped) => Some(true),
                        None => None,
                    },
                    match_url.clone(),
                    critical_error,
                ),
                // Send to submission that represents p2
                get_error_report(
                    format!("[{}]({})", p1, p1_issue),
                    error_msg,
                    match fault.clone() {
                        Some(PlayerType::Regular) => Some(true),
                        Some(PlayerType::Flipped) => Some(false),
                        None => None,
                    },
                    match_url,
                    critical_error,
                ),
            );
        }
        None => (
            format!(
                "[{}] Opponent: [{}]({}) &#124; [Match]({})",
                if winner_id.clone().unwrap() == p1 {
                    "WIN"
                } else {
                    "LOSS"
                },
                p2,
                p2_issue,
                match_url.clone().unwrap()
            ),
            format!(
                "[{}] Opponent: [{}]({}) &#124; [Match]({})",
                if winner_id.unwrap() == p2 {
                    "WIN"
                } else {
                    "LOSS"
                },
                p1,
                p1_issue,
                match_url.unwrap()
            ),
        ),
    }
}

fn get_error_report(
    opponent_issue_link: String,
    error_msg: String,
    fault: Option<bool>,
    match_url: Option<String>,
    critical_error: bool,
) -> String {
    let mut output = match fault {
        Some(false) => format!("[WIN] Opponent: {}", opponent_issue_link),
        Some(true) => format!(
            "[ERROR] Opponent: {}\n**Error:**\n{}\n\n{}",
            opponent_issue_link,
            error_msg,
            if critical_error {
                "This submission has been disqualififed"
            } else {
                ""
            }
        ),
        None => format!(
            "[UNKNOWN FAULT] Opponent: {}\n**Error:**\n{}\n\n{}",
            opponent_issue_link,
            error_msg,
            if critical_error {
                "Both submissions have been disualified"
            } else {
                ""
            }
        ),
    };
    if match_url.is_some() && !critical_error {
        output.push_str(&format!("\n[Match]({})", match_url.unwrap()));
    }
    return output;
}
