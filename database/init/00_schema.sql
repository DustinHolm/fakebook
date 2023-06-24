CREATE TABLE app_user (
    user_id         SERIAL                      PRIMARY KEY,
    first_name      VARCHAR(128)                NOT NULL,
    last_name       VARCHAR(128)                NOT NULL
);

CREATE TABLE user_relation (
    user_id_a       INTEGER                     NOT NULL REFERENCES app_user (user_id), 
    user_id_b       INTEGER                     NOT NULL REFERENCES app_user (user_id),
    PRIMARY KEY     (user_id_a, user_id_b),
    CONSTRAINT      positive_integers           CHECK (user_id_a > 0),
    CONSTRAINT      a_less_than_b               CHECK (user_id_a < user_id_b)
);

CREATE INDEX ON user_relation (user_id_a);
CREATE INDEX ON user_relation (user_id_b);

CREATE TABLE post (
    post_id         SERIAL                      PRIMARY KEY,
    author          INTEGER                     NOT NULL REFERENCES app_user (user_id),
    created_on      TIMESTAMP WITH TIME ZONE    NOT NULL,
    content         TEXT                        NOT NULL
);

CREATE TABLE comment (
    comment_id      SERIAL                      PRIMARY KEY,
    referenced_post INTEGER                     NOT NULL REFERENCES post (post_id),
    author          INTEGER                     NOT NULL REFERENCES app_user (user_id),
    created_on      TIMESTAMP WITH TIME ZONE    NOT NULL,
    content         TEXT                        NOT NULL
);
