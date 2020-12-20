CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_post
        FOREIGN KEY(post_id)
            REFERENCES posts(id)
            ON DELETE CASCADE
)