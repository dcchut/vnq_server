table! {
    quotes (id) {
        id -> Integer,
        content -> Text,
        votes -> Integer,
        visible -> Bool,
        moderated_by -> Nullable<Integer>,
        ip -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        salt -> Text,
        password -> Text,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(quotes -> users (user_id));

allow_tables_to_appear_in_same_query!(quotes, users,);
