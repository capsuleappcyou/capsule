CREATE TABLE capsule_users
(
    id        SERIAL PRIMARY KEY,
    name      VARCHAR   NOT NULL,
    create_at timestamp NOT NULL
)