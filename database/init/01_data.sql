BEGIN;

SELECT setseed(0.12345);

INSERT INTO app_user ("user_id", "first_name", "last_name")
SELECT
    i,
    concat('First', i),
    concat('Last', i)
FROM generate_series(1, 10000) AS s(i);

ALTER SEQUENCE app_user_user_id_seq RESTART WITH 10001;

INSERT INTO post ("post_id", "author", "created_on", "content")
SELECT
    i,
    ceil(random() * 10000),
    current_timestamp(0) - (random() * (interval '2 years')),
    concat(md5(random()::text), ' ', md5(random()::text), ' ', md5(random()::text))
FROM generate_series(1, 100000) AS s(i);

ALTER SEQUENCE post_post_id_seq RESTART WITH 100001;

INSERT INTO comment ("comment_id", "referenced_post", "author", "created_on", "content")
SELECT
    i,
    ceil(random() * 100000),
    ceil(random() * 10000),
    current_timestamp(0) - (random() * (interval '2 years')),
    md5(random()::text)
FROM generate_series(1, 100000) AS s(i);

ALTER SEQUENCE comment_comment_id_seq RESTART WITH 10001;

INSERT INTO user_relation ("user_id_a", "user_id_b")
SELECT
    ceil(random() * 5000),
    ceil(random() * 5000 + 5000)
FROM generate_series(1, 100000) AS s(i)
ON CONFLICT DO NOTHING;

COMMIT;