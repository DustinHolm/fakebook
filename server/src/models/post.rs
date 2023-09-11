use time::OffsetDateTime;

pub struct Post {
    post_id: i32,
    author: i32,
    created_on: OffsetDateTime,
    content: String,
}
