use rand::Rng;

use crate::backend::models::submission_model::Submission;

pub(crate) fn create_match_making_queue(
    submissions: Vec<Submission>,
) -> Vec<(Submission, Submission)> {
    let mut submissions = submissions;
    // Remove all submissions of disqualified bots
    submissions = submissions
        .into_iter()
        .filter(|submission| submission.disqualified == 0)
        .collect();

    if submissions.len() < 2 {
        return vec![];
    }

    // Sort from lowest to highest
    submissions.sort_by(|a, b| (a.mmr.round() as i32).cmp(&(b.mmr.round() as i32)));

    let least_played_bots = find_least_played(&submissions);
    let mut match_queue: Vec<(Submission, Submission)> = Vec::new();
    let mut match_count = 10;
    for submission in least_played_bots {
        if match_count <= 0 {
            break;
        }
        if let Ok(pair) = pair_close_bots(submission, &submissions) {
            match_queue.push(pair);
            match_count -= 1;
        }
    }

    return match_queue;
}

fn pair_close_bots(
    submission: Submission,
    sorted_submissions_by_mmr: &Vec<Submission>,
) -> Result<(Submission, Submission), String> {
    let submission_index = match get_index_of_submission(&submission, sorted_submissions_by_mmr) {
        Ok(index) => index,
        Err(error_msg) => return Err(error_msg),
    };

    let mut random_number = submission_index + rand::thread_rng().gen_range(0..10) - 5;
    if random_number == 0 {
        random_number += 1;
    }
    random_number = std::cmp::max(0, random_number);

    let length_index = (sorted_submissions_by_mmr.len() - 1).try_into();
    match length_index {
        Ok(length_index) => {
            random_number = std::cmp::min(random_number, length_index);
        }
        Err(error) => {
            return Err(format!("{}", error));
        }
    }

    // Wo don't allow bot to play against another bot of the same author
    if sorted_submissions_by_mmr[random_number as usize].user == submission.user {
        return Err("Don't allow bots to play against same owner".to_string());
    }

    Ok((
        submission.clone(),
        sorted_submissions_by_mmr[random_number as usize].clone(),
    ))
}

fn find_least_played(submissions: &Vec<Submission>) -> Vec<Submission> {
    let mut submissions = submissions.clone();

    // Return a list of the bots listed in order of played matches
    submissions.sort_by(|a, b| a.matches_played.cmp(&b.matches_played));
    return submissions;
}

fn get_index_of_submission(
    submission: &Submission,
    sorted_submissions_by_mmr: &Vec<Submission>,
) -> Result<i32, String> {
    let mut submission_index = 0;
    for i in 0..sorted_submissions_by_mmr.len() {
        if submission.id == sorted_submissions_by_mmr[i].id {
            submission_index = i;
            break;
        }
    }

    let submission_index = submission_index.try_into();
    match submission_index {
        Ok(submission_index) => {
            return Ok(submission_index);
        }
        Err(error) => {
            return Err(format!("{}", error));
        }
    }
}
