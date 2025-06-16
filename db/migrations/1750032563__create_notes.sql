CREATE TABLE notes (
    id uuid PRIMARY KEY default gen_random_uuid(),
    text varchar(255) NOT NULL
);