CREATE TABLE password_reset_request
(
    token    VARCHAR(255) PRIMARY KEY NOT NULL,
    user_id  VARCHAR(255)             NOT NULL,
    datetime TIMESTAMP                NOT NULL
);