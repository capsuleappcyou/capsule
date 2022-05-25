CREATE TABLE capsule_users
(
    id        SERIAL PRIMARY KEY,
    user_name VARCHAR(200) NOT NULL,
    create_at timestamp    NOT NULL
);
create unique index capsule_users_name_uindex on capsule_users (user_name);