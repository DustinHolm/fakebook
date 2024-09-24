use std::time::Duration;

use async_graphql::{
    connection::{Edge, EmptyFields},
    Context, Subscription, ID,
};
use async_stream::stream;
use time::OffsetDateTime;
use tokio::time::interval;
use tokio_stream::Stream;
use tracing::{error, instrument};

use crate::{
    domain::{
        app_user::AppUser,
        db_id::{CanDecodeId, DbId},
        errors::GqlError,
        post::Post,
        relay_meta::AppCursor,
    },
    infrastructure::{
        db::{Loaders, Repo},
        notification_center::{ListenerTopic, Notification, NotificationCenter},
        DbError,
    },
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
        let repo = ctx.data::<Repo>().map_err(|_| GqlError::InternalData)?;

        let user_id =
            AppUser::decode(&user_id).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let mut interval = interval(Duration::from_secs(10));
        let mut last_seen = OffsetDateTime::now_utc();

        let stream = stream!({
            loop {
                let posts: Result<Vec<Edge<AppCursor, Post, EmptyFields>>, DbError> = repo
                    .query(
                        "SELECT * FROM post WHERE author = ANY($1) AND created_on > $2",
                        &[&user_id, &last_seen],
                        |rows| {
                            rows.into_iter()
                                .map(|row| {
                                    let post: Post = row.try_into()?;
                                    Ok(Edge::new(AppCursor(post.post_id), post))
                                })
                                .collect::<Result<_, DbError>>()
                        },
                    )
                    .await;

                if let Ok(posts) = posts {
                    if !posts.is_empty() {
                        yield posts;
                    }

                    last_seen = OffsetDateTime::now_utc();
                    let _ = ctx.data::<Loaders>().map(|loaders| loaders.clear_caches());
                };

                interval.tick().await;
            }
        });

        Ok(stream)
    }

    #[instrument(skip(self, ctx), err)]
    async fn home_feed<'a>(
        &'a self,
        ctx: &'a Context<'a>,
    ) -> Result<impl Stream<Item = Vec<Edge<AppCursor, Post, EmptyFields>>> + 'a, GqlError> {
        let repo = ctx.data::<Repo>().map_err(|_| GqlError::InternalData)?;
        let notification_center = ctx
            .data::<NotificationCenter>()
            .map_err(|_| GqlError::InternalData)?;

        let user_id = DbId::from(1); // Placeholder until we have auth

        let friend_ids: Vec<DbId> = repo
            .query(
                r"
                    SELECT user_id_b AS friend_id
                    FROM user_relation 
                    WHERE user_id_a = $1 
                    UNION
                    SELECT user_id_a AS friend_id
                    FROM user_relation
                    WHERE user_id_b = $1
                ",
                &[&user_id],
                |rows| {
                    rows.into_iter()
                        .map(|row| {
                            let friend_id = row.try_get("friend_id").map_err(DbError::mapping)?;
                            Ok::<_, DbError>(friend_id)
                        })
                        .collect()
                },
            )
            .await
            .map_err(|e| {
                error!(message = e.to_string());
                GqlError::DbLoad
            })?;

        let mut author_ids = friend_ids;
        author_ids.push(user_id);

        let topics = author_ids
            .into_iter()
            .map(|id| ListenerTopic::User(id))
            .collect();
        let mut handle = notification_center.subscribe(topics).await;

        let stream = stream!({
            while let Some(notifications) = handle.receive().await {
                let post_ids: Vec<DbId> = notifications
                    .into_iter()
                    .filter_map(|n| {
                        if let Notification::Post(post) = n {
                            Some(post.post_id)
                        } else {
                            None
                        }
                    })
                    .collect();

                let posts: Result<Vec<Edge<AppCursor, Post, EmptyFields>>, DbError> = repo
                    .query(
                        "SELECT * FROM post WHERE post_id = ANY($1)",
                        &[&post_ids],
                        |rows| {
                            rows.into_iter()
                                .map(|row| {
                                    let post: Post = row.try_into()?;
                                    Ok(Edge::new(AppCursor(post.post_id), post))
                                })
                                .collect::<Result<_, DbError>>()
                        },
                    )
                    .await;

                if let Ok(posts) = posts {
                    if !posts.is_empty() {
                        yield posts;
                    }

                    let _ = ctx.data::<Loaders>().map(|loaders| loaders.clear_caches());
                };
            }
        });

        Ok(stream)
    }
}
