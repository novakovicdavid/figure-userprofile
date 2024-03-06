create table "user"
(
    id       text not null
        constraint user_pk
            primary key,

    email    text not null
        constraint email_check
            check (email = lower(email)),

    password text not null,
    role     text not null
);

create unique index user_email_uindex
    on "user" (email);

create table profile
(
    id              text not null
        constraint profile_pk
            primary key,

    username        text not null,
    display_name    text,

    user_id         text not null
        constraint profile_user_id_fk
            references "user",

    profile_picture text,
    bio             text,
    banner          text
);

create unique index profile_username_uindex
    on profile (username);

create table figure
(
    id          text    not null,
    title       text    not null,
    width       integer not null,
    height      integer not null,
    profile_id  text    not null
        constraint figure_profile_id_fk
            references profile,

    url         text    not null,
    description text
);
