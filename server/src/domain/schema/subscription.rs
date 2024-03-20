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
use tracing::{error, instrument};

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
                if let Ok(posts) = poll_posts(pool, &[user_id], &last_seen).await {
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

    #[instrument(skip(self, ctx), err)]
    async fn home_feed<'a>(
        &'a self,
        ctx: &'a Context<'a>,
    ) -> Result<impl Stream<Item = Vec<Edge<AppCursor, Post, EmptyFields>>> + 'a, GqlError> {
        let pool = ctx.data::<Pool>().map_err(|_| GqlError::InternalData)?;

        let user_id = DbId::from(1); // Placeholder until we have auth

        let conn = pool.get().await.map_err(|e| {
            error!(message = e.to_string());
            GqlError::InternalData
        })?;

        let stmt = conn
            .prepare_cached(
                r"
                    SELECT user_id_b AS friend_id
                    FROM user_relation 
                    WHERE user_id_a = $1 
                    UNION
                    SELECT user_id_a AS friend_id
                    FROM user_relation
                    WHERE user_id_b = $1
                ",
            )
            .await
            .map_err(|e| {
                error!(message = e.to_string());
                GqlError::DbLoad
            })?;

        let friend_ids = conn
            .query(&stmt, &[&user_id])
            .await
            .map_err(|e| {
                error!(message = e.to_string());
                GqlError::DbLoad
            })?
            .into_iter()
            .map(|row| {
                let friend_id: i32 = row.try_get("friend_id")?;
                let friend_id = DbId::from(friend_id);
                Ok(friend_id)
            })
            .collect::<Result<Vec<_>, tokio_postgres::Error>>()
            .map_err(|e| {
                error!(message = e.to_string());
                GqlError::DbLoad
            })?;

        let mut author_ids = friend_ids;
        author_ids.push(user_id);

        let mut interval = interval(Duration::from_secs(10));
        let mut last_seen = OffsetDateTime::now_utc();

        let stream = stream!({
            loop {
                if let Ok(posts) = poll_posts(pool, &author_ids, &last_seen).await {
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
    user_ids: &[DbId],
    last_seen: &OffsetDateTime,
) -> Result<Vec<Edge<AppCursor, Post, EmptyFields>>, DbError> {
    let conn = pool.get().await?;

    let stmt = conn
        .prepare_cached("SELECT * FROM post WHERE author = ANY($1) AND created_on > $2")
        .await
        .map_err(DbError::statement)?;

    let rows = conn
        .query(&stmt, &[&user_ids, &last_seen])
        .await
        .map_err(DbError::statement)?;

    rows.into_iter()
        .map(|row| {
            let post: Post = row.try_into()?;
            Ok(Edge::new(AppCursor(post.post_id), post))
        })
        .collect()
}
