CREATE TABLE "profile" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES "user" (id),
    bio TEXT NOT NULL
);
