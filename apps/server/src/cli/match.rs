use std::{fs, time::Duration};

use clap::ArgMatches;

use crate::{
    external_related::readme_factory::{
        create_and_encode_file, generate_gif_from_turn, get_match_from_tiles_compact,
    },
    game::{
        game_state::{ErrorType, GameConfig, GameResult},
        initialize_game::initialize_game,
        player::PlayerType,
    },
};

pub(crate) fn run_local_match(matches: &ArgMatches) {
    let script1 = std::fs::read_to_string(matches.get_one::<String>("path-one").unwrap())
        .expect("Could not load script 1");
    let script2 = std::fs::read_to_string(matches.get_one::<String>("path-two").unwrap())
        .expect("Could not load script 2");
    let outdir = matches.get_one::<String>("output").unwrap();

    let mut config = GameConfig::new();
    config.live_print_match = *matches.get_one::<bool>("print").unwrap();
    config.bot_initialization_timeout =
        Duration::from_millis(*matches.get_one::<u64>("init-timeout").unwrap());
    config.bot_turn_timeout =
        Duration::from_millis(*matches.get_one::<u64>("turn-timeout").unwrap());

    let (results, turns, logger) = initialize_game(&script1, &script2, config);

    // Create output folder
    fs::create_dir_all(outdir).unwrap();

    if *matches.get_one::<bool>("readme").unwrap() {
        let mut file: String = "".to_string();
        file.push_str(&format!(
            "<div align=\"center\"><p>{}</p></div>\n\n",
            match results {
                GameResult::PlayerOneWon => "Script 1 (🟩) won",
                GameResult::PlayerTwoWon => "Script 2 (🟥) won",
                _ => "",
            }
        ));

        if let GameResult::Error(error) = results.clone() {
            let string: String;
            file.push_str(&format!(
                "<div align=\"center\"><p>--- Match has errors ---</p>\n\n{}</div>",
                match error {
                    ErrorType::GameDeadlock => "Reason for error: Game Deadlock",
                    ErrorType::GameError { reason, fault } => {
                        // TODO This feels very hacky, there should be another solution for this
                        string = print_error(Some(reason), fault);
                        &string
                    }
                    ErrorType::RuntimeError { reason, fault } => {
                        string = print_error(Some(reason), fault);
                        &string
                    }
                    ErrorType::TurnTimeout { fault } => {
                        string = print_error(None, fault);
                        &string
                    }
                }
            ))
        }
        file.push_str(&get_match_from_tiles_compact(turns.clone()));

        fs::write(format!("./{}/{}", outdir, "match.md"), file)
            .expect("Could not write match file");
    }

    if *matches.get_one::<bool>("gif").unwrap() {
        let (color_palette, images) = generate_gif_from_turn(turns, Some(results), 50);
        create_and_encode_file(
            format!("./{}/{}", outdir, "match.gif".to_string()),
            images,
            &color_palette,
            50,
        );
    }

    if *matches.get_one::<bool>("log").unwrap() {
        let mut file: String = "".to_string();
        for log in logger {
            file.push_str(&format!("{}\n", log));
        }
        fs::write(format!("./{}/{}", outdir, "match.log.txt"), file)
            .expect("Could not write log file");
    }
}

fn print_error(reason: Option<String>, fault: Option<PlayerType>) -> String {
    format!(
        "Reason for error: {}\n\nFault: {}",
        &reason.unwrap_or("Unknown".to_string()),
        match fault {
            Some(PlayerType::Flipped) => "Player 1 (🟩)",
            Some(PlayerType::Regular) => "Player 2 (🟥)",
            _ => "Unknown",
        }
    )
}
