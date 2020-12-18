CREATE TABLE comments (
    id SERIAL NOT NULL PRIMARY KEY,
    post_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_post
        FOREIGN KEY(post_id)
            REFERENCES posts(id)
            ON DELETE CASCADE
)