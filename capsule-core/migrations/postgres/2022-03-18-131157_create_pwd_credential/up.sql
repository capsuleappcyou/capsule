CREATE TABLE capsule_user_credentials
(
    id              SERIAL PRIMARY KEY,
    user_name       VARCHAR(200) NOT NULL,
    credential_name VARCHAR(100) NOT NULL,
    flat_data       TEXT         NOT NULL,
    create_at       timestamp    NOT NULL
);
create unique index capsule_user_credentials_user_name_credential_name_uindex on capsule_user_credentials (user_name, credential_name);