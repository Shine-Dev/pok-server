table! {
    comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Uuid,
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
