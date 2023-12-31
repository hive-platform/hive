-- Your SQL goes here
create table firmwares
(
    id                 varchar   default nanoid() not null
        constraint firmwares_pk
            primary key,
    name               varchar(255)               not null,
    description        varchar(4096),
    created_date       timestamp default now()    not null,
    updated_date       timestamp,
    deleted_date       timestamp,
    created_by_user_id varchar
        constraint firmwares_created_users_id_fk
            references users
            on update cascade on delete set null,
    updated_by_user_id varchar
        constraint firmwares_updated_users_id_fk
            references users
            on update cascade on delete set null,
    deleted_by_user_id varchar
        constraint firmwares_deleted_users_id_fk
            references users
            on update cascade on delete set null,
    is_deleted         boolean   default false    not null,
    is_activated       boolean   default false    not null
);

