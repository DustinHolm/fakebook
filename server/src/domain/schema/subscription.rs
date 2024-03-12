use std::time::Duration;

use async_graphql::{
    connection::{Edge, EmptyFields},
    Context, Subscription, ID,
};
use async_stream::stream;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio::time::interval;
use tokio_stream::Stream;
use tracing::instrument;

use crate::{
    domain::{
        app_user::AppUser,
        db_id::{CanDecodeId, DbId},
        errors::{DbError, GqlError},
        post::Post,
        relay_meta::AppCursor,
    },
    infrastructure::db::Loaders,
};

pub struct RootSubscription;

#[Subscription]
impl RootSubscription {
    #[instrument(skip(self, ctx), err)]
    async fn user_feed<'a>(
        &'a self,
        ctx: &'a Context<'a>,
        user_id: ID,
    ) -> Result<impl Stream<Item = Vec<Edge<AppCursor, Post, EmptyFields>>> + 'a, GqlError> {
        let pool = ctx.data::<Pool>().map_err(|_| GqlError::InternalData)?;

        let user_id =
            AppUser::decode(&user_id).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let mut interval = interval(Duration::from_secs(10));
        let mut last_seen = OffsetDateTime::now_utc();

        let stream = stream!({
            loop {
                if let Ok(posts) = poll_posts(pool, &user_id, &last_seen).await {
                    if !posts.is_empty() {
                        yield posts;
                    }

                    last_seen = OffsetDateTime::now_utc();
                    let _ = ctx.data::<Loaders>().map(|loaders| loaders.clear_caches());
                    interval.tick().await;
                };
            }
        });

        Ok(stream)
    }
}

#[instrument(skip(pool), err)]
async fn poll_posts(
    pool: &Pool,
    user_id: &DbId,
    last_seen: &OffsetDateTime,
) -> Result<Vec<Edge<AppCursor, Post, EmptyFields>>, DbError> {
    let conn = pool.get().await?;

    let stmt = conn
        .prepare_cached("SELECT * FROM post WHERE author = $1 AND created_on > $2")
        .await
        .map_err(DbError::statement)?;

    let rows = conn
        .query(&stmt, &[&user_id, &last_seen])
        .await
        .map_err(DbError::statement)?;

    rows.into_iter()
        .map(|row| {
            let post: Post = row.try_into()?;
            Ok(Edge::new(AppCursor(post.post_id), post))
        })
        .collect()
}
