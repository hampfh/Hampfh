#[cfg(test)]
mod tests {
    use crate::game::{
        game::Wall,
        player::{Player, PlayerType},
        sandbox::sandbox_executor::create_lua_game_object,
        tests::util::{test_std, test_std_bool},
    };

    #[test]
    fn out_of_bounds_works() {
        test_std_bool(
            vec![
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(0,0)"), false),
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(-1,-1)"), true),
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(9,9)"), true),
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(8,8)"), false),
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(8,-100)"), true),
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(8,0.5)"), false),
                (format!("[] STD__CHECK_OUT_OF_BOUNDS(8,-100)"), true),
            ],
            None,
        );
    }

    #[test]
    fn occupied_works() {
        let game_context = context_player_close();
        test_std(
            vec![
                format!("opponent = STD__OCCUPIED({}, 4, 4)", game_context),
                format!("player = STD__OCCUPIED({}, 4, 5)", game_context),
                format!("outside1 = STD__OCCUPIED({}, 4, 6)", game_context),
                format!("wall = STD__OCCUPIED({}, 0, 0)", game_context),
                format!("outofbounds = STD__OCCUPIED({}, 9, 9)", game_context),
                format!("outside2 = STD__OCCUPIED({}, 8, 0)", game_context),
            ],
            |ctx| {
                assert_eq!(ctx.globals().get::<_, bool>("opponent").unwrap(), true);
                assert_eq!(ctx.globals().get::<_, bool>("player").unwrap(), true);
                assert_eq!(ctx.globals().get::<_, bool>("outside1").unwrap(), false);
                assert_eq!(ctx.globals().get::<_, bool>("wall").unwrap(), true);
                assert_eq!(ctx.globals().get::<_, bool>("outofbounds").unwrap(), true);
                assert_eq!(ctx.globals().get::<_, bool>("outside2").unwrap(), false);
                Ok(())
            },
        );
    }

    #[test]
    fn player_occupied_works() {
        test_std_bool(
            vec![
                (format!("[] STD__PLAYER_OCCUPIED([c],4,4)"), true),
                (format!("[] STD__PLAYER_OCCUPIED([c],4,5)"), false),
                (format!("[] STD__PLAYER_OCCUPIED([c],100,100)"), false),
                (format!("[] STD__PLAYER_OCCUPIED([c],-200,0)"), false),
            ],
            Some(context_player_close()),
        );
    }

    #[test]
    fn jump_test() {
        test_std_bool(
            vec![(format!("[] STD__JUMP_POSSIBLE([c],0)"), true)],
            Some(context_player_close()),
        );
        let close_game_all_blocked = create_lua_game_object(
            vec![
                Wall {
                    x1: 3,
                    y1: 4,
                    x2: 3,
                    y2: 5,
                },
                Wall {
                    x1: 5,
                    y1: 4,
                    x2: 5,
                    y2: 5,
                },
                Wall {
                    x1: 4,
                    y1: 2,
                    x2: 4,
                    y2: 3,
                },
            ],
            true,
            Player {
                player_type: PlayerType::Flipped,
                x: 4,
                y: 5,
                wall_count: 0,
            },
            Player {
                player_type: PlayerType::Regular,
                x: 4,
                y: 4,
                wall_count: 0,
            },
        );
        test_std_bool(
            vec![(format!("[] STD__JUMP_POSSIBLE([c],0)"), false)],
            Some(close_game_all_blocked),
        );
        let closed_with_left_open = create_lua_game_object(
            vec![
                Wall {
                    x1: 5,
                    y1: 4,
                    x2: 5,
                    y2: 5,
                },
                Wall {
                    x1: 4,
                    y1: 2,
                    x2: 4,
                    y2: 3,
                },
            ],
            true,
            Player {
                player_type: PlayerType::Flipped,
                x: 4,
                y: 5,
                wall_count: 0,
            },
            Player {
                player_type: PlayerType::Regular,
                x: 4,
                y: 4,
                wall_count: 0,
            },
        );
        test_std_bool(
            vec![(format!("[] STD__JUMP_POSSIBLE([c],0)"), true)],
            Some(closed_with_left_open),
        );
    }

    #[test]
    fn get_tile_test() {
        test_std(
            vec![
                format!("wall = STD__GET_TILE({},0,0)", context_player_close()),
                format!("p2 = STD__GET_TILE({},4,4)", context_player_close()),
                format!("p1 = STD__GET_TILE({},4,5)", context_player_close()),
                format!("nothing1 = STD__GET_TILE({},8,8)", context_player_close()),
                format!("nothing2 = STD__GET_TILE({},3,3)", context_player_close()),
            ],
            |ctx| {
                assert_eq!(ctx.globals().get::<_, String>("wall").unwrap(), "3");
                assert_eq!(ctx.globals().get::<_, String>("p2").unwrap(), "2");
                assert_eq!(ctx.globals().get::<_, String>("p1").unwrap(), "1");
                assert_eq!(ctx.globals().get::<_, String>("nothing1").unwrap(), "0");
                assert_eq!(ctx.globals().get::<_, String>("nothing2").unwrap(), "0");
                Ok(())
            },
        );
    }

    fn context_player_close() -> String {
        return create_lua_game_object(
            vec![Wall {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 1,
            }],
            true,
            Player {
                player_type: PlayerType::Flipped,
                x: 4,
                y: 5,
                wall_count: 0,
            },
            Player {
                player_type: PlayerType::Regular,
                x: 4,
                y: 4,
                wall_count: 0,
            },
        );
    }
}
