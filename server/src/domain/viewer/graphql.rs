use crate::{
    domain::{db_id::DbId, errors::GqlError, relay_meta::paginate},
    infrastructure::{logging::current_span_as_headers, urls::Urls},
};
use crate::{
    domain::{post::Post, relay_meta::AppConnection, viewer::Viewer},
    infrastructure::db::Loaders,
};
use async_graphql::{Context, Object};
use reqwest::Client;
use serde::Deserialize;
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
    pub async fn relevant_posts(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<AppConnection<Post>, GqlError> {
        let loaders = ctx.data::<Loaders>()?;

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

        let connection = paginate(after, before, first, last, posts).await?;

        Ok(connection)
    }

    #[instrument(skip(self, ctx), err)]
    pub async fn relevant_ad_url(&self, ctx: &Context<'_>) -> Result<String, GqlError> {
        let client = ctx.data::<Client>()?;
        let urls = ctx.data::<Urls>()?;

        let response = client
            .get(&urls.ad_service_ad_link)
            .headers(current_span_as_headers())
            .send()
            .await
            .map_err(|e| GqlError::OtherServer(e.to_string()))?;

        response
            .json::<AdLink>()
            .await
            .map(|d| d.ad_link)
            .map_err(|e| GqlError::OtherServer(e.to_string()))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdLink {
    ad_link: String,
}
