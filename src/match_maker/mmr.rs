use pathfinding::num_traits::ToPrimitive;

use super::constants::DEFAULT_MATCH_GAIN;

pub(crate) struct MMR {
    rating: f32,
    matches_played: i32,
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

pub(crate) fn calculate_mmr(p1_mmr: MMR, p2_mmr: MMR, p1_winner: bool) -> (f32, f32) {
    let (max, min) = get_max_and_min(p1_mmr.rating, p2_mmr.rating);
    let p1_highest = p1_mmr.rating > p2_mmr.rating;

    let mmr_diff = match p1_highest == p1_winner {
        true => min / max,  // low diff
        false => max / min, // high diff
    };

    let p1_new_mmr = DEFAULT_MATCH_GAIN * mmr_diff * calc_decay(p1_mmr.matches_played);
    let p2_new_mmr = DEFAULT_MATCH_GAIN * mmr_diff * calc_decay(p2_mmr.matches_played);
    return (p1_new_mmr, p2_new_mmr);
}
