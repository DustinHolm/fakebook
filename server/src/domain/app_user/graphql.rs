use async_graphql::{Context, InputObject, Object, ID};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use tracing::instrument;

use crate::{
    domain::{
        errors::GqlError,
        post::Post,
        relay_meta::{paginate, AppConnection},
    },
    infrastructure::db::Loaders,
};

use super::domain::{AppUser, SUFFIX};

#[Object]
impl AppUser {
    pub async fn id(&self) -> ID {
        let combined = self.user_id.to_string() + SUFFIX;

        ID(URL_SAFE.encode(combined))
    }

    pub async fn first_name(&self) -> &str {
        &self.first_name
    }

    pub async fn last_name(&self) -> &str {
        &self.last_name
    }

    #[instrument(skip_all, err)]
    #[graphql(complexity = "10 * child_complexity")]
    // TODO: Limit number of loaded friends?
    pub async fn friends(&self, ctx: &Context<'_>) -> Result<Vec<AppUser>, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        let friend_ids = loaders
            .friend_id
            .load_one(self.user_id)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))?;

        let users = loaders
            .app_user
            .load_many(friend_ids)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .into_values()
            .collect();

        Ok(users)
    }

    #[instrument(skip_all, err)]
    #[graphql(
        complexity = "first.unwrap_or(0).try_into().unwrap_or(usize::MAX) * child_complexity 
        + last.unwrap_or(0).try_into().unwrap_or(usize::MAX) * child_complexity"
    )]
    pub async fn posts(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<AppConnection<Post>, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        let posts = loaders
            .posts_of_author
            .load_one(self.user_id)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))?;

        let connection = paginate(after, before, first, last, posts)
            .await
            .map_err(|_| GqlError::DbLoad)?;

        Ok(connection)
    }
}

#[derive(InputObject, Debug)]
pub struct AppUserInput {
    pub(in crate::domain) first_name: String,
    pub(in crate::domain) last_name: String,
}

#[derive(InputObject, Debug)]
pub struct AddFriendInput {
    pub(in crate::domain) friend: ID,
}
