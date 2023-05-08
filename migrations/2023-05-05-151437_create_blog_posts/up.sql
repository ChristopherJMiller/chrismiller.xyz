CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  post_url TEXT NOT NULL,
  posted TIMESTAMP NOT NULL,
  image_url TEXT,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  plaintext_body TEXT NOT NULL
);

CREATE UNIQUE INDEX posts_url ON posts(post_url);
