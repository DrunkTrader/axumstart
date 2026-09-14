CREATE TABLE "post_tag" (
    post_id INTEGER NOT NULL REFERENCES "post" (id),
    tag_id INTEGER NOT NULL REFERENCES "tag" (id),
    PRIMARY KEY (post_id, tag_id)
);
