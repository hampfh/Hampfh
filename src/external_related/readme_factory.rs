use chrono::NaiveDateTime;
use diesel::SqliteConnection;

use crate::backend;
use crate::backend::models::match_model::Match;
use crate::backend::models::submission_model::Submission;
use crate::backend::models::turn_model::Turn;
use crate::backend::models::user_model::User;
use crate::game::board::{board_from_string, Tile};

use std::fs;

use super::repo_updater::get_issue_url;

pub fn clear_match_dir() {
    match std::fs::remove_dir_all("data") {
        Ok(_) => (),
        Err(error) => println!("Could not clear data dir, reason: {}", error),
    };
    match std::fs::create_dir("data") {
        Ok(_) => (),
        Err(error) => println!("Could not create data dir, reason: {}", error),
    }
    match std::fs::create_dir("data/matches") {
        Ok(_) => (),
        Err(error) => println!("Could not create data/matches dir, reason: {}", error),
    }
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
        "{}<br/>{}{}{}{}",
        get_readme_header(),
        get_last_turn_of_last_match(players.clone(), submissions.clone(), &matches, turns),
        generate_score_board(&submissions, &players),
        format!(
            "🕹 [Match log](./data/match_log.md) &#124; [Submission log](./data/submission_log.md) 🤖"
        ),
        credits(chrono::Local::now().naive_local())
    );
}

fn get_readme_header() -> String {
    return String::from(
"<div align=\"center\">
<h3>🎉🎉🎉 Welcome to the scripting game! 🎉🎉🎉</h3>
<img src=\"https://img.shields.io/badge/-BETA-yellow\"/>
<img src=\"https://img.shields.io/github/issues-closed-raw/hampfh/hampfh/challenger?color=limegreen&label=Bots\"/>
<img src=\"https://img.shields.io/badge/-lua-darkblue\">
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
        return format!("No matches yet...\n\n");
    }

    let last_turn = &turns[turns.len() - 1];
    let (winner, loser) = match get_players_from_turn(last_turn, &players, &submissions, &matches) {
        Some(players) => players,
        None => {
            return format!("");
        }
    };

    return format!(
        "<div align=\"center\"><p>Latest game:</p><p><a href=\"https://github.com/{}\">@{}</a> vs <a href=\"https://github.com/{}\">@{}</a></p>\n<a href=\"./data/matches/{}.md\">Go to match</a></div>\n\n---\n\n{}\n---\n",
        winner.username,
        winner.username,
        loser.username,
        loser.username,
        matches.iter().find(|current| current.id == last_turn.match_id).unwrap().id,
        generate_board(board_from_string(last_turn.board.clone()))
    );
}

#[allow(dead_code)]
fn generate_board(board: Vec<Tile>) -> String {
    let mut output = String::from("\n<div align=\"center\">\n");

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

    output.push_str("</div>\n");

    return output;
}

pub(crate) fn build_match_log_wrapper() {
    let conn = backend::db::establish_connection().get().unwrap();
    match write_file(
        "./data/match_log.md",
        create_match_log(&conn, &Match::list(&conn)),
    ) {
        Ok(_) => (),
        Err(e) => println!("Error writing match log: {}", e),
    }
}

fn create_match_log(conn: &SqliteConnection, matches: &Vec<Match>) -> String {
    let mut output = format!("<div align=\"center\">\n\n# Matches\n");
    let mut matches = matches.clone();
    matches.reverse();
    // Get matches
    for current in matches {
        let result = current.players(&conn);
        if result.is_none() {
            continue;
        }
        let ((winner, winner_submission), (loser, loser_submission)) = result.unwrap();
        output.push_str(&format!(
            "<p>\n\n{} vs {}</p>\n<p>@{} vs @{}</p>\n<p><a href=\"./matches/{}.md\">Match</a></p>\n<p>{}</p>\n\n---\n",
            format!(
                "[{}]({})",
                winner_submission.id,
                get_issue_url(winner_submission.issue_number)
            ),
            format!(
                "[{}]({})",
                loser_submission.id,
                get_issue_url(loser_submission.issue_number)
            ),
            winner.username,
            loser.username,
            current.id,
            current.created_at.format("%Y-%m-%d %H:%M:%S")
        ));
    }
    output.push_str("</div>\n");
    return output;
}

pub(crate) fn build_submission_log_wrapper() {
    let conn = backend::db::establish_connection().get().unwrap();
    match write_file(
        "./data/submission_log.md",
        create_submission_log(&conn, &Submission::list(&conn)),
    ) {
        Ok(_) => (),
        Err(e) => println!("Error writing submission log: {}", e),
    }
}
fn create_submission_log(conn: &SqliteConnection, submissions: &Vec<Submission>) -> String {
    let mut output = format!("<div align=\"center\">\n\n# Submissions\n");

    let mut submissions = submissions.clone();
    submissions.sort_by(|a, b| (a.mmr.round() as i32).cmp(&(b.mmr.round() as i32)));
    submissions.reverse();

    // Get submissions
    for current in submissions {
        let user = User::by_id(&current.user, &conn);
        if user.is_none() {
            continue;
        };
        output.push_str(&format!(
            "<p>MMR: {} &#124; @{} &#124; <a href=\"{}\">{}</a> {} &#124; {}</p>  \n",
            current.mmr.round(),
            user.unwrap().username,
            current.issue_url,
            current.id,
            if current.disqualified >= 1 {
                "❌"
            } else {
                "✅"
            },
            current.created_at.format("%Y-%m-%d %H:%M")
        ));
    }
    output.push_str("</div>");
    return output;
}

fn generate_score_board(submissions: &Vec<Submission>, players: &Vec<User>) -> String {
    if submissions.len() <= 0 || players.len() <= 0 {
        return format!("");
    }

    let mut sorted_submissions = submissions.clone();
    sorted_submissions.sort_by(|a, b| (b.mmr.round() as i32).cmp(&(a.mmr.round() as i32)));
    sorted_submissions = sorted_submissions.into_iter().filter(|current| current.disqualified == 0).collect();

    let mut output = format!(
        "<div align=\"center\">\n\n| MMR | (Top 10) | Submission  |\n| :-- | --: | :--: |\n"
    );

    // Limit to only top 10
    for i in 0..std::cmp::min(10, sorted_submissions.len()) {
        let user = players
            .iter()
            .find(|current| current.id == sorted_submissions[i].user);
        output.push_str(&format!(
            "| {} | {} | [Submission]({}) {} |\n",
            sorted_submissions[i].mmr.round().to_string(),
            match user {
                Some(user) => user.username.clone(),
                None => format!("<Unknown>"),
            },
            sorted_submissions[i].issue_url,
            if sorted_submissions[i].disqualified >= 1 {
                "❌"
            } else {
                "✅"
            }
        ))
    }
    output.push_str("\n");
    return output;
}

pub fn build_match_files_wrapper() {
    let conn = backend::db::establish_connection().get().unwrap();
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
    let p1_is_winner = target_match.p1_is_winner;
    let (winner_color, loser_color) = if p1_is_winner == 1 {
        ("🟩", "🟥")
    } else {
        ("🟥", "🟩")
    };

    let mut file = format!(
        "<div align=\"center\"><h1>{} vs {}</h1><p><a href=\"{}\">{} {}</a> vs <a href=\"{}\">{} {}</a></p>\n<p>Winner: {}</p></div>\n\n---\n",
        winner.username, loser.username, win_sub.issue_url, winner_color, win_sub.id, los_sub.issue_url, los_sub.id, loser_color, winner_color, 
    );
    if target_match.match_error.is_some() {
        file.push_str(&format!("<div align=\"center\"><p>{}</p></div>\n\n", target_match.match_error.as_ref().unwrap()));
    }
    file.push_str(&get_match_from_tiles(
        turns
            .unwrap()
            .iter()
            .map(|turn| board_from_string(turn.board.clone()))
            .collect(),
    ));

    return Some(file);
}

pub(crate) fn get_match_from_tiles(turns: Vec<Vec<Tile>>) -> String {
    let mut output = String::new();
    let mut round = 1;
    for turn in turns {
        output.push_str(&format!("<div align=\"center\">Round {}</div><br/>", round));
        output.push_str(&generate_board(turn));
        output.push_str(&format!("\n---\n\n"));
        round += 1;
    }
    return output;
}

fn credits(last_updated: NaiveDateTime) -> String {
    let spacing = format!("&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
    return format!(
        "<br/><div align=\"center\"><a href=\"https://www.craft.do/s/geS8o08lvJ4cfD\">What is this? </a> {}&#124;{} Hampus Hallkvist {}&#124;{} {}</div>",
        spacing,
        spacing,
        spacing,
        spacing,
        &last_updated.format("%Y-%m-%d %H:%M")
    );
}
