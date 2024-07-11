use crate::game::game_state::{Move, Wall, MAP_SIZE};

/// Convert lua string to Move enum
///
/// Converts a string like ["x1,y1,x2,y2" -> Wall]
pub(crate) fn deserialize_wall(input: &str) -> Move {
    let splits = input.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
    if splits.len() != 4 as usize {
        return Move::Invalid {
            reason: format!("Invalid return format, expected 4 values, got: [{}]", input),
        };
    }
    let result = splits
        .iter()
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>().unwrap_or_else(|_| -1))
        .collect::<Vec<i32>>();

    // If any of the values are invalid (negative), the move is invalid
    if result.iter().any(|x| *x < 0 || *x >= MAP_SIZE) {
        return Move::Invalid {
            reason: "Invalid wall param".to_string(),
        };
    }

    // Same coordinates
    if result[0] == result[2] && result[1] == result[3] {
        return Move::Invalid {
            reason: "Overlapping wall coordinates".to_string(),
        };
    }

    return Move::Wall(Wall {
        x1: result[0],
        y1: result[1],
        x2: result[2],
        y2: result[3],
    });
}

#[cfg(test)]
mod tests {
    use crate::game::game_state::{Move, Wall};

    use super::deserialize_wall;

    #[test]
    fn test_deserialize_wall_correct() {
        assert_eq!(
            deserialize_wall("1,2,3,4"),
            Move::Wall(Wall {
                x1: 1,
                y1: 2,
                x2: 3,
                y2: 4
            })
        );
    }

    #[test]
    fn successfully_reject_letters_or_symbols() {
        assert_eq!(
            deserialize_wall("1,2,3,a"),
            Move::Invalid {
                reason: "Invalid wall param".to_string()
            }
        );
    }

    #[test]
    fn successfully_reject_invalid_coordinates() {
        assert_eq!(
            deserialize_wall("1,2,3,-1"),
            Move::Invalid {
                reason: "Invalid wall param".to_string()
            }
        );
        assert_eq!(
            deserialize_wall("0,2,3,9"),
            Move::Invalid {
                reason: "Invalid wall param".to_string()
            }
        );
    }

    #[test]
    fn successfully_reject_deserialize_invalid_wall() {
        assert_eq!(
            deserialize_wall("1,2,3"),
            Move::Invalid {
                reason: "Invalid return format, expected 4 values, got: [1,2,3]".to_string()
            }
        );
        assert_eq!(
            deserialize_wall("1,2,3,4,5"),
            Move::Invalid {
                reason: "Invalid return format, expected 4 values, got: [1,2,3,4,5]".to_string()
            }
        );
        assert_eq!(
            deserialize_wall(",,,"),
            Move::Invalid {
                reason: "Invalid wall param".to_string()
            }
        );
        assert_eq!(
            deserialize_wall("0,0,0,0"),
            Move::Invalid {
                reason: "Overlapping wall coordinates".to_string()
            }
        );
    }
}
