// @generated automatically by Diesel CLI.

diesel::table! {
    Matches (id) {
        id -> Text,
        winner -> Text,
        loser -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        p1_is_winner -> Integer,
        match_error -> Nullable<Text>,
    }
}

diesel::table! {
    Submissions (id) {
        id -> Text,
        user -> Text,
        script -> Text,
        comment -> Nullable<Text>,
        wins -> Integer,
        issue_url -> Text,
        issue_number -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        disqualified -> Integer,
        mmr -> Float,
        matches_played -> Integer,
    }
}

diesel::table! {
    Turns (id) {
        id -> Text,
        match_id -> Text,
        turn -> Integer,
        board -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    Users (id) {
        id -> Text,
        username -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(Submissions -> Users (user));
diesel::joinable!(Turns -> Matches (match_id));

diesel::allow_tables_to_appear_in_same_query!(
    Matches,
    Submissions,
    Turns,
    Users,
);
