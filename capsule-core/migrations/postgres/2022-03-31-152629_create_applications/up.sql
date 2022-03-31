CREATE TABLE capsule_applications
(
    id                    serial primary key,
    application_name      varchar(200) not null,
    application_directory varchar(200) not null,
    create_at             timestamp    not null
);
create index capsule_applications_application_name_index on capsule_applications (application_name);

