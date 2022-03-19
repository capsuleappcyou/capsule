CREATE TABLE capsule_user_password_credentials
(
    id         SERIAL PRIMARY KEY,
    user_name  VARCHAR(200) NOT NULL,
    hash_value TEXT         NOT NULL,
    salt       INTEGER      NOT NULL,
    create_at  timestamp    NOT NULL
);