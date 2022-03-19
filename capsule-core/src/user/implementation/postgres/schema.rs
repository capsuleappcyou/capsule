table! {
    capsule_user_password_credentials (id) {
        id -> Int4,
        user_name -> Varchar,
        hash_value -> Text,
        salt -> Int4,
        create_at -> Timestamp,
    }
}

table! {
    capsule_users (id) {
        id -> Int4,
        user_name-> Varchar,
        create_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    capsule_user_password_credentials,
    capsule_users,
);
