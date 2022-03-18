CREATE TABLE capsule_users
(
    id        SERIAL PRIMARY KEY,
    name      VARCHAR2(200)   NOT NULL,
    create_at timestamp NOT NULL
);
create unique index capsule_users_name_uindex on capsule_users (name);