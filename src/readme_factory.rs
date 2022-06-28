use crate::db::models::match_model::Match;
use crate::db::models::submission_model::Submission;
use crate::db::models::turn_model::Turn;
use crate::db::models::user_model::User;
use crate::game::board::{board_from_string, Tile};

use std::fs;

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
        get_last_turn_of_last_match(players.clone(), submissions.clone(), matches, turns),
        generate_score_board(&submissions, &players),
        create_history_table(submissions)
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
    matches: Vec<Match>,
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

fn create_history_table(submissions: Vec<Submission>) -> String {
    let mut output = format!("<div align=\"center\">\n\n| Challenger submissions  |\n| :--: |\n");

    let mut submissions = submissions;
    submissions.reverse();
    for submission in submissions {
        output.push_str(&format!(
            "| &#124; [Submission]({}) &#124; {} |\n",
            submission.issue_url,
            submission.created_at.format("%Y-%m-%d %H:%M")
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
