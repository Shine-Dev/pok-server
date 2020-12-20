table! {
    comments (id) {
        id -> Int8,
        post_id -> Int8,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int8,
        title -> Text,
        content -> Text,
        latitude -> Numeric,
        longitude -> Numeric,
        created_at -> Timestamp,
    }
}

joinable!(comments -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
