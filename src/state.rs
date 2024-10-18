use axum_macros::FromRef;
use leptos::LeptosOptions;
use poise::serenity_prelude as serenity;
use std::{ops::Deref, sync::Arc};

#[derive(Clone, FromRef, Debug)]
pub struct DiscordClient {
    pub http: Arc<serenity::Http>,
    pub cache: Arc<serenity::Cache>,
    pub shard_manager: Arc<serenity::ShardManager>,
}

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub discord_client: DiscordClient,
}

impl<C> From<C> for DiscordClient
where
    C: Deref<Target = serenity::Client>,
{
    fn from(client: C) -> Self {
        Self {
            http: client.http.clone(),
            cache: client.cache.clone(),
            shard_manager: client.shard_manager.clone(),
        }
    }
}

impl AppState {
    pub fn new(
        leptos_options: LeptosOptions,
        discord_client: impl Deref<Target = serenity::Client>,
    ) -> Self {
        Self {
            leptos_options,
            discord_client: discord_client.into(),
        }
    }
}
