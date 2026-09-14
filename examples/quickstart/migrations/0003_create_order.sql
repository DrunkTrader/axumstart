CREATE TABLE "order" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES "user" (id),
    total_cents BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
);
