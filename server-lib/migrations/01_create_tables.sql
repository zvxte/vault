CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    username VARCHAR(16) NOT NULL,
    password VARCHAR(128) NOT NULL,
    salt BYTEA NOT NULL,
    created_at BIGINT NOT NULL,
    connected_at BIGINT NOT NULL
);

CREATE TABLE passwords (
    password_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    domain_name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password BYTEA NOT NULL,
    nonce BYTEA NOT NULL
);

CREATE TABLE notes (
    note_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    title BYTEA,
    title_nonce BYTEA,
    content BYTEA,
    content_nonce BYTEA
);

CREATE TABLE sessions (
    session_id BYTEA PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id)
);
