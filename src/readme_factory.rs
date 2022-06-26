use crate::db::models::match_model::Match;
use crate::db::models::submission_model::Submission;
use crate::db::models::turn_model::Turn;
use crate::db::models::user_model::User;
use crate::game::board::Tile;

use std::fs;

pub fn write_file(path: &str, data: String) -> std::io::Result<()> {
    fs::write(path, data)
}

pub fn generate_readme(
    _: Vec<User>,
    submissions: Vec<Submission>,
    __: Vec<Match>,
    ___: Vec<Turn>,
) -> String {
    return format!(
        "{}{}",
        get_readme_header(),
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
<br/>
<br/>");
}

#[allow(dead_code)]
fn generate_board(board: Vec<Tile>) -> String {
    let mut output = String::from("\n\n---\n");

    let mut count = 0;
    for tile in board {
        output.push_str(match tile {
            Tile::Empty => "⬜️",
            Tile::P1 => "🟩",
            Tile::P2 => "🟥",
            Tile::Wall => "⬛️",
        });
        if count % 9 == 0 {
            println!("<br>");
        }
        count += 1;
    }

    output.push_str("\n---\n");

    return output;
}

fn create_history_table(submissions: Vec<Submission>) -> String {
    let mut output = format!("<div align=\"center\">\n\n| Challenger submissions  |\n| :--: |\n");

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
