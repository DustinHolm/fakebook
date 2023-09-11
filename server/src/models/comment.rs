use time::OffsetDateTime;

pub struct Comment {
    comment_id: i32,
    referenced_post: i32,
    author: i32,
    created_on: OffsetDateTime,
    content: String,
}
