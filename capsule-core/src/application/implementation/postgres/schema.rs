table! {
    capsule_applications (id) {
        id -> Int4,
        application_id -> BigInt,
        application_name -> Varchar,
        owner -> Varchar,
        create_at -> Timestamp,
    }
}
