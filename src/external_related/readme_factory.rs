use chrono::NaiveDateTime;
use diesel::SqliteConnection;
use gif::{Encoder, Frame, Repeat};

use crate::backend;
use crate::backend::models::match_model::Match;
use crate::backend::models::submission_model::Submission;
use crate::backend::models::turn_model::Turn;
use crate::backend::models::user_model::User;
use crate::game::board::{board_from_string, Tile};
use crate::game::game::{GameResult, MAP_SIZE};

use std::borrow::Cow;
use std::fs::{self, File};

use super::repo_updater::get_issue_url;

pub fn clear_match_dir() {
    if let Err(error) = std::fs::remove_dir_all("data") {
        println!("Could not clear data dir, reason: {}", error)
    };
    if let Err(error) = std::fs::create_dir("data") {
        println!("Could not create data dir, reason: {}", error)
    }
    if let Err(error) = std::fs::create_dir("data/matches") {
        println!("Could not create data/matches dir, reason: {}", error)
    }
    if let Err(error) = fs::create_dir("data/gifs") {
        println!("Could not create data/gifs dir, reason: {}", error);
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
    let is_live = std::env::var("LIVE").unwrap_or("false".to_string()) == "true";
    let url_prepend = if is_live { "" } else { "." };

    let image_scale: u16 = 50;
    let selected_matches = pick_front_page_matches(&matches, &turns);
    let selection_count = selected_matches.len();
    render_matches_to_gif(&selected_matches, image_scale);

    return format!(
            "{}<br/>  <div align=\"center\">\n\n| {} | {} | {} |  \n| :--: | :--: | :--: |  \n|{}|{}|{}|  \n</div>{}{}{}",
            get_readme_header(),
            get_match_header(&selected_matches[0].0, &players, &submissions),
            if selection_count > 1 { get_match_header(&selected_matches[1].0, &players, &submissions) } else { String::from("") },
            if selection_count > 2 { get_match_header(&selected_matches[2].0, &players, &submissions) } else { String::from("") },
            format!("<img style=\"margin: 10px\" src=\"{}/data/gifs/one.gif?raw=true\" width=\"{}\" height=\"{}\" />", url_prepend, 200, 200),
            if selection_count > 1 { format!("<img style=\"margin: 10px\" src=\"{}/data/gifs/two.gif?raw=true\" width=\"{}\" height=\"{}\" />", url_prepend, 200, 200) } else { String::from("") },
            if selection_count > 2 { format!("<img style=\"margin: 10px\" src=\"{}/data/gifs/three.gif?raw=true\" width=\"{}\" height=\"{}\" />", url_prepend, 200, 200) } else { String::from("") },
    /*         get_last_turn_of_last_match(players.clone(), submissions.clone(), &matches, turns), */
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
<img src=\"https://img.shields.io/badge/-BETA-yellow\"/>
<img src=\"https://img.shields.io/github/issues-closed-raw/hampfh/hampfh/challenger?color=limegreen&label=Bots\"/>
<img src=\"https://img.shields.io/badge/-lua-darkblue\">
</div>

<div align=\"center\"> 
	<h3>🤖🧑‍💻🤖 <a href=\"https://github.com/Hampfh/Hampfh/issues/new?assignees=&labels=challenger&template=challenger-submission-template.md&title=%5BChallenger-submission%5D\">Create your challenger</a>  🤖🧑‍💻🤖</h3>
</div>
");
}

fn get_match_header(
    selected_match: &Match,
    players: &Vec<User>,
    submissions: &Vec<Submission>,
) -> String {
    let m1_result = get_players_from_turn(&selected_match, &players, &submissions);
    if let Some((user1, user2)) = m1_result {
        return format!(
            "<a href=\"{}\">{}</a> vs <a href=\"{}\">{}</a><br/>  <a href=\"{}\">Match</a>  ",
            format!("https://github.com/{}", user1.username),
            user1.username,
            format!("https://github.com/{}", user2.username),
            user2.username,
            format!("./matches/{}.md", selected_match.id)
        );
    }
    return String::new();
}

#[allow(dead_code)]
fn get_players_from_turn(
    selected_match: &Match,
    players: &Vec<User>,
    submissions: &Vec<Submission>,
) -> Option<(User, User)> {
    let winning_submission = submissions
        .iter()
        .find(|current| current.id == selected_match.winner);

    let loosing_submission = submissions
        .iter()
        .find(|current| current.id == selected_match.loser);

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

/**
 * Pick 3 matches to render to gifs, try to take the latest matches
 * but if they aren't longer than 5 moves then attempt to take
 * other matches that are at least 5 moves long.
 */
fn pick_front_page_matches(
    matches: &Vec<Match>,
    turns: &Vec<Turn>,
) -> Vec<(Match, Vec<Turn>, i32)> {
    let mut processed_matches: Vec<(Match, Vec<Turn>, i32)> = vec![];
    let turn_iter = turns.iter();

    let mut not_short_matches = 0;
    for current_match in matches.iter().rev() {
        if not_short_matches >= 3 {
            break;
        }
        let id = current_match.id.clone();
        let rounds = turn_iter
            .clone()
            .filter(|turn| turn.match_id == id)
            .map(|turn| turn.clone());

        let round_count = rounds.clone().count();
        processed_matches.push((
            current_match.to_owned(),
            rounds.collect(),
            round_count as i32,
        ));
        if round_count > 5 {
            not_short_matches += 1;
        }
    }

    // Side effect here, write to file
    processed_matches.sort_by(|a, b| b.2.cmp(&a.2));
    return processed_matches;
}

fn render_matches_to_gif(matches_to_render: &Vec<(Match, Vec<Turn>, i32)>, image_scale: u16) {
    let mut counter = 0;

    for (match_to_render, turns, _) in matches_to_render.iter().take(3) {
        render_match_gif(
            match_to_render.to_owned(),
            turns,
            format!(
                "./data/gifs/{}.gif",
                match counter {
                    0 => "one",
                    1 => "two",
                    _ => "three",
                }
            ),
            image_scale,
        );
        counter += 1;
    }
}

fn render_match_gif(match_to_render: Match, turns: &Vec<Turn>, render_path: String, scale: u16) {
    let last_match_turns = turns
        .iter()
        .filter(|turn| turn.match_id == match_to_render.id)
        .collect::<Vec<&Turn>>();

    let (color_palette, last_match_image) = generate_gif_from_turn(
        last_match_turns
            .iter()
            .map(|turn| board_from_string(turn.board.clone()))
            .collect(),
        Some(GameResult::PlayerOneWon),
        scale,
    );
    create_and_encode_file(render_path, last_match_image, &color_palette, scale);
}

fn get_string_from_tile(tile: Tile) -> String {
    return match tile {
        Tile::Empty => String::from("⬜️"),
        Tile::P1 => String::from("🟩"),
        Tile::P2 => String::from("🟥"),
        Tile::Wall => String::from("⬛️"),
    };
}

#[allow(dead_code)]
fn generate_board(board: Vec<Tile>) -> String {
    let mut output = String::from("\n<div align=\"center\">\n");

    let mut count = 1;
    for tile in board {
        output.push_str(&get_string_from_tile(tile));
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
    sorted_submissions = sorted_submissions
        .into_iter()
        .filter(|current| current.disqualified == 0)
        .collect();

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
        let build_result = build_match_with_players(
            Match::get_players(&current.id, &conn),
            Match::get_turns(&current.id, &conn),
            &current,
        );
        match match build_result {
            Some(file) => write_file(&format!("data/matches/{}.md", current.id), file),
            None => Ok(()),
        } {
            Ok(()) => (),
            Err(_) => (),
        }
    }
}

fn build_match_with_players(
    player_result: Option<((User, Submission), (User, Submission))>,
    turns_result: Option<Vec<Turn>>,
    target_match: &Match,
) -> Option<String> {
    if player_result.is_none() {
        return None;
    }
    let ((winner, win_sub), (loser, los_sub)) = player_result.unwrap();

    if turns_result.is_none() {
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
        file.push_str(&format!(
            "<div align=\"center\"><p>{}</p></div>\n\n",
            target_match.match_error.as_ref().unwrap()
        ));
    }
    file.push_str(
        &get_match_from_tiles(
            turns_result
                .unwrap()
                .iter()
                .map(|turn| board_from_string(turn.board.clone()))
                .collect(),
        )
        .as_str(),
    );

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
pub(crate) fn get_match_from_tiles_compact(turns: Vec<Vec<Tile>>) -> String {
    let mut output = String::new();
    for i in (0..turns.len()).step_by(3) {
        let mut rows: Vec<String> = vec![];

        let left = turns[i].clone();
        let mid = if i + 1 < turns.len() {
            Some(turns[i + 1].clone())
        } else {
            None
        };
        let right = if i + 2 < turns.len() {
            Some(turns[i + 2].clone())
        } else {
            None
        };

        for j in 0..MAP_SIZE {
            let mut row: String = "".to_string();
            let tile_index = |column: i32| (j * MAP_SIZE + column) as usize;
            for k in 0..MAP_SIZE {
                let left_tile = left[tile_index(k)];
                row.push_str(&get_string_from_tile(left_tile));
            }

            if let Some(mid_tile) = mid.clone() {
                row.push_str(" ");
                for k in 0..MAP_SIZE {
                    row.push_str(&get_string_from_tile(mid_tile[tile_index(k)]));
                }
            }

            if let Some(right_tile) = right.clone() {
                row.push_str(" ");
                for k in 0..MAP_SIZE {
                    row.push_str(&get_string_from_tile(right_tile[tile_index(k)]));
                }
            }
            rows.push(row);
        }
        output.push_str(&format!(
            "<div align=\"center\">\n{}</div><br/>",
            rows.join("\n  ")
        ));
    }
    return output;
}

pub(crate) fn generate_gif_from_turn(
    turns: Vec<Vec<Tile>>,
    game_result: Option<GameResult>,
    image_scale: u16,
) -> ([u8; 12], Vec<Vec<u8>>) {
    #[rustfmt::skip]
    let color_map = [
        0xFF,   0xFF,   0xFF,   // White
        0x00,   0xAF,   0x00,   // Green
        0xEF,   0x01,   0x08,   // Red
        0x00,   0x00,   0x00,   // Black
    ];
    let mut beacon_states: Vec<Vec<u8>> = vec![];

    for turn in &turns {
        let mut current_image: Vec<u8> = vec![];
        for y in 0..(MAP_SIZE as u16 * image_scale) {
            for x in 0..(MAP_SIZE as u16 * image_scale) {
                let tile = turn[(y / image_scale * MAP_SIZE as u16 + x / image_scale) as usize];
                current_image.push(match tile {
                    Tile::Empty => 0,
                    Tile::P1 => 1,
                    Tile::P2 => 2,
                    Tile::Wall => 3,
                });
            }
        }
        beacon_states.push(current_image);
    }
    if game_result.is_some() && turns.len() > 5 {
        let mut win_screen: Vec<u8> = vec![];
        for _ in 0..(MAP_SIZE * MAP_SIZE * image_scale as i32 * image_scale as i32) {
            win_screen.push(match game_result.as_ref().unwrap() {
                GameResult::PlayerOneWon => 1,
                GameResult::PlayerTwoWon => 2,
                GameResult::Error(_) => 3,
            });
        }
        for _ in 0..3 {
            beacon_states.push(win_screen.clone());
        }
    }
    return (color_map, beacon_states);
}
pub(crate) fn create_and_encode_file(
    path: String,
    beacon_states: Vec<Vec<u8>>,
    color_map: &[u8; 12],
    image_scale: u16,
) {
    let image_size = MAP_SIZE as u16 * image_scale;
    let mut image = File::create(path).unwrap();
    let mut encoder = Encoder::new(&mut image, image_size, image_size, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for state in &beacon_states {
        let mut frame = Frame::default();
        frame.delay = if beacon_states.len() < 5 { 50 } else { 20 };
        frame.width = image_size;
        frame.height = image_size;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
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
