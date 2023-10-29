-- Add migration script here
CREATE TABLE IF NOT EXISTS chat_messages (
    id SERIAL PRIMARY KEY,
    message varchar(500) NOT NULL,
    sender_id bigint NOT NULL,
    receiver_id bigint NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE
);

