use crate::domain::{db_id::DbId, errors::GqlError, relay_meta::paginate};
use crate::{
    domain::{post::Post, relay_meta::AppConnection, viewer::Viewer},
    infrastructure::db::Loaders,
};
use async_graphql::{Context, Object};
use tracing::{error, instrument};

#[Object]
impl Viewer {
    pub async fn first_name(&self) -> &str {
        &self.user.first_name
    }

    pub async fn last_name(&self) -> &str {
        &self.user.last_name
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(
        complexity = "first.unwrap_or(0).try_into().unwrap_or(usize::MAX) * child_complexity 
        + last.unwrap_or(0).try_into().unwrap_or(usize::MAX) * child_complexity"
    )]
    async fn relevant_posts(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<AppConnection<Post>, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|e| {
            error!(message = e.message);
            GqlError::InternalData
        })?;

        let id = DbId::from(1); // Placeholder until we have auth

        let friends = loaders
            .friend_id
            .load_one(id)
            .await
            .map_err(|e| {
                error!(message = e.to_string());
                GqlError::DbLoad
            })?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))?;

        let mut authors = friends;
        authors.push(id);

        let mut posts: Vec<Post> = loaders
            .posts_of_author
            .load_many(authors)
            .await
            .map_err(|e| {
                error!(message = e.to_string());
                GqlError::DbLoad
            })?
            .drain()
            .flat_map(|(_, posts)| posts)
            .collect();

        posts.sort_unstable_by_key(|p| p.created_on);

        let connection = paginate(after, before, first, last, posts)
            .await
            .map_err(|e| {
                error!(message = e.message);
                GqlError::InternalData
            })?;

        Ok(connection)
    }
}
