table! {
    capsule_user_credentials (id) {
        id -> Int4,
        user_name -> Varchar,
        credential_name -> Varchar,
        flat_data -> Text,
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
    capsule_user_credentials,
    capsule_users,
);
