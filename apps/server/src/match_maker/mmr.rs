use pathfinding::num_traits::ToPrimitive;

use super::constants::DEFAULT_MATCH_GAIN;

pub(crate) struct MMR {
    pub(crate) rating: f32,
    pub(crate) matches_played: i32,
}

fn get_max_and_min(p1_mmr: f32, p2_mmr: f32) -> (f32, f32) {
    let max = if p1_mmr > p2_mmr { p1_mmr } else { p2_mmr };
    let min = if p1_mmr < p2_mmr { p1_mmr } else { p2_mmr };
    return (max, min);
}

/// Calculate a decary value between 1 and 0.5,
/// the more matches the more decay the value will be.
/// (The value will level out after ~40 matches)
fn calc_decay(matches_played: i32) -> f32 {
    1.5 - std::f64::consts::E
        .powf((matches_played / 10).into())
        .to_f32()
        .unwrap()
        / (std::f64::consts::E.powf((matches_played / 10).into()) + 1.0)
            .to_f32()
            .unwrap()
}

fn enforce_floor(mmr: f32) -> f32 {
    if mmr < 0.0 {
        return 0.0;
    }
    return mmr;
}

pub(crate) fn calculate_mmr(
    p1_mmr: MMR,
    p2_mmr: MMR,
    p1_winner: bool,
    amplifier: f32,
) -> (f32, f32) {
    let (max, min) = get_max_and_min(p1_mmr.rating, p2_mmr.rating);
    let p1_highest = p1_mmr.rating > p2_mmr.rating;

    // If the MMR wins we lower the mmr diff otherwise, increase it
    let mmr_diff = match p1_highest == p1_winner {
        true => min / max,  // low diff
        false => max / min, // high diff
    };

    match p1_winner {
        true => {
            return (
                enforce_floor(
                    p1_mmr.rating
                        + DEFAULT_MATCH_GAIN
                            * amplifier
                            * mmr_diff
                            * calc_decay(p1_mmr.matches_played),
                ),
                enforce_floor(
                    p2_mmr.rating
                        - DEFAULT_MATCH_GAIN
                            * amplifier
                            * mmr_diff
                            * calc_decay(p2_mmr.matches_played),
                ),
            );
        }
        false => {
            return (
                enforce_floor(
                    p1_mmr.rating
                        - DEFAULT_MATCH_GAIN
                            * amplifier
                            * mmr_diff
                            * calc_decay(p1_mmr.matches_played),
                ),
                enforce_floor(
                    p2_mmr.rating
                        + DEFAULT_MATCH_GAIN
                            * amplifier
                            * mmr_diff
                            * calc_decay(p2_mmr.matches_played),
                ),
            );
        }
    }
}
