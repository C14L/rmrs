table! {
    pushtoken (id) {
        id -> Int4,
        token -> Varchar,
        created -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    sr (id) {
        id -> Varchar,
        name -> Varchar,
        // created -> Date,
        url -> Varchar,
        over18 -> Bool,
        lang -> Varchar,
        title -> Varchar,
        header_title -> Varchar,
        display_name -> Varchar,
        subreddit_type -> Varchar,
        subscribers -> Int4,
        subscribers_here -> Int4,
    }
}
