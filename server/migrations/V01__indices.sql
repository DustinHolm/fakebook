CREATE INDEX IF NOT EXISTS index_post_author ON post (author);
CREATE INDEX IF NOT EXISTS index_comment_author ON comment (author);
CREATE INDEX IF NOT EXISTS index_comment_post ON comment (referenced_post);
