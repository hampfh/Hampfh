use diesel::SqliteConnection;

use crate::db;
use crate::db::models::match_model::Match;
use crate::db::models::submission_model::Submission;
use crate::db::models::turn_model::Turn;
use crate::db::models::user_model::User;
use crate::game::board::{board_from_string, Tile};

use std::fs;

pub fn clear_match_dir() {
    std::fs::remove_dir_all("data/matches").unwrap();
    std::fs::create_dir("data/matches").unwrap();
}

pub fn write_file(path: &str, data: String) -> std::io::Result<()> {
    fs::write(path, data)
}

pub fn generate_readme(
    players: Vec<User>,
    submissions: Vec<Submission>,
    matches: Vec<Match>,
    turns: Vec<Turn>,
) -> String {
    return format!(
        "{}<br/><br/>{}<br/><br/>{}<br/><br/>{}",
        get_readme_header(),
        get_last_turn_of_last_match(players.clone(), submissions.clone(), &matches, turns),
        generate_score_board(&submissions, &players),
        create_history_table(&submissions, &matches)
    );
}

fn get_readme_header() -> String {
    return String::from(
"<div align=\"center\"> <h1>Hampus Hallkvist</h1>
<h3>🎉🎉🎉 Welcome to my github profile 🎉🎉🎉</h3>
</div>

<div align=\"center\"> 
	<h3>🤖🧑‍💻🤖 <a href=\"https://github.com/Hampfh/Hampfh/issues/new?assignees=&labels=challenger&template=challenger-submission-template.md&title=%5BChallenger-submission%5D\">Create your challenger</a>  🤖🧑‍💻🤖</h3>
</div>
");
}

fn get_players_from_turn(
    turn: &Turn,
    players: &Vec<User>,
    submissions: &Vec<Submission>,
    matches: &Vec<Match>,
) -> Option<(User, User)> {
    let last_match = matches.iter().find(|current| current.id == turn.match_id);
    if last_match.is_none() {
        return None;
    }

    let winning_submission = submissions
        .iter()
        .find(|current| current.id == last_match.unwrap().winner);

    let loosing_submission = submissions
        .iter()
        .find(|current| current.id == last_match.unwrap().loser);

    if winning_submission.is_none() || loosing_submission.is_none() {
        return None;
    }

    let winner = players
        .iter()
        .find(|current| current.id == winning_submission.unwrap().user)
        .unwrap();

    let loser = players
        .iter()
        .find(|current| current.id == loosing_submission.unwrap().user)
        .unwrap();

    return Some((winner.to_owned(), loser.to_owned()));
}

fn get_last_turn_of_last_match(
    players: Vec<User>,
    submissions: Vec<Submission>,
    matches: &Vec<Match>,
    turns: Vec<Turn>,
) -> String {
    if turns.len() == 0 {
        return format!("No matches yet...");
    }

    let last_turn = &turns[turns.len() - 1];
    let (winner, loser) = match get_players_from_turn(last_turn, &players, &submissions, &matches) {
        Some(players) => players,
        None => {
            return format!("");
        }
    };

    return format!(
        "<div align=\"center\"><p>Latest game:</p><p>{} vs {}</p></div>\n{}",
        winner.username,
        loser.username,
        generate_board(board_from_string(last_turn.board.clone()))
    );
}

#[allow(dead_code)]
fn generate_board(board: Vec<Tile>) -> String {
    let mut output = String::from("\n\n---\n<div align=\"center\">\n");

    let mut count = 1;
    for tile in board {
        output.push_str(match tile {
            Tile::Empty => "⬜️",
            Tile::P1 => "🟩",
            Tile::P2 => "🟥",
            Tile::Wall => "⬛️",
        });
        if count % 9 == 0 {
            output.push_str("<br>");
        }
        count += 1;
    }

    output.push_str("</div>\n\n---\n");

    return output;
}

fn create_history_table(bor_submissions: &Vec<Submission>, bor_matches: &Vec<Match>) -> String {
    let conn = db::db::establish_connection().get().unwrap();
    let mut match_list = format!("<details><summary>Matches</summary>  \n");
    let mut submission_list = format!("<details><summary>Submissions</summary>  \n");

    let mut matches = bor_matches.clone();
    matches.reverse();
    let mut submissions = bor_submissions.clone();
    submissions.reverse();

    // Get matches
    for current in matches {
        let result = current.players(&conn);
        if result.is_none() {
            continue;
        }
        let ((winner, _), (loser, _)) = result.unwrap();
        match_list.push_str(&format!(
            "<p>{} vs {} &#124; <a href=\"./data/matches/{}.md\">Match</a></p>  \n",
            winner.username, loser.username, current.id
        ));
    }
    match_list.push_str("</details>\n");

    // Get submissions
    for current in submissions {
        let user = User::by_id(&current.user, &conn);
        if user.is_none() {
            continue;
        }
        submission_list.push_str(&format!(
            "<p>{} &#124; <a href=\"{}\">Submission</a> &#124; {}</p>  \n",
            user.unwrap().username,
            current.issue_url,
            current.created_at.format("%Y-%m-%d %H:%M")
        ));
    }
    submission_list.push_str("</details>  \n");

    let mut output = format!("<div align=\"center\">\n\n<table><tr><td>Matches played</td><td>Challenger submissions</td></tr><tr><td>{}</td><td>{}</td></tr></table></div>", match_list, submission_list);

    output.push_str("</div>");

    return output;
}

fn generate_score_board(submissions: &Vec<Submission>, players: &Vec<User>) -> String {
    if submissions.len() <= 0 || players.len() <= 0 {
        return format!("");
    }

    let mut sorted_submissions = submissions.clone();
    sorted_submissions.sort_by(|a, b| b.score.cmp(&a.score));

    let mut output = format!(
        "<div align=\"center\">\n\n| Scoreboard | (Top 10) | Submission  |\n| :-- | --: | :--: |\n"
    );

    // Limit to only top 10
    for i in 0..std::cmp::min(10, sorted_submissions.len()) {
        let user = players
            .iter()
            .find(|current| current.id == sorted_submissions[i].user);
        output.push_str(&format!(
            "| {} | {} | [Submission]({}) |\n",
            sorted_submissions[i].score.to_string(),
            match user {
                Some(user) => user.username.clone(),
                None => format!("<Unknown>"),
            },
            sorted_submissions[i].issue_url
        ))
    }
    output.push_str("\n");
    return output;
}

pub fn build_match_files_wrapper() {
    let conn = db::db::establish_connection().get().unwrap();
    build_match_files(&conn, Match::list(&conn));
}
fn build_match_files(conn: &SqliteConnection, matches: Vec<Match>) {
    for current in matches {
        let build_result = build_match(conn, &current);
        match match build_result {
            Some(file) => write_file(&format!("data/matches/{}.md", current.id), file),
            None => Ok(()),
        } {
            Ok(()) => (),
            Err(_) => (),
        }
    }
}

fn build_match(conn: &SqliteConnection, target_match: &Match) -> Option<String> {
    let result = Match::get_players(&target_match.id, &conn);
    if result.is_none() {
        return None;
    }
    let ((winner, win_sub), (loser, los_sub)) = result.unwrap();

    let turns = Match::get_turns(&target_match.id, &conn);
    if turns.is_none() {
        return None;
    }

    let mut file = format!(
        "<div align=\"center\"><h1>{} vs {}</h1><p><a href=\"{}\">Submission</a> vs <a href=\"{}\">Submission</a></p></div>",
        winner.username, loser.username, win_sub.issue_url, los_sub.issue_url
    );
    let mut round = 1;
    for turn in turns.unwrap() {
        file.push_str(&format!("<div align=\"center\">Round {}</div>", round));
        file.push_str(&generate_board(board_from_string(turn.board)));
        file.push_str(&format!("\n---\n\n"));
        round += 1;
    }

    return Some(file);
}
