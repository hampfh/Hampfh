table! {
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

table! {
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

table! {
    Turns (id) {
        id -> Text,
        match_id -> Text,
        turn -> Integer,
        board -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    Users (id) {
        id -> Text,
        username -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(Submissions -> Users (user));
joinable!(Turns -> Matches (match_id));

allow_tables_to_appear_in_same_query!(Matches, Submissions, Turns, Users,);
