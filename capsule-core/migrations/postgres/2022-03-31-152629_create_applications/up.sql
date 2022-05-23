CREATE TABLE capsule_applications
(
    id               serial primary key,
    application_id   bigint not null,
    application_name varchar(200) not null,
    owner            varchar(200) not null,
    create_at        timestamp    not null
);

create unique index capsule_applications_application_name_uindex on capsule_applications (application_name);
create unique index capsule_applications_application_id_uindex on capsule_applications (application_id);
