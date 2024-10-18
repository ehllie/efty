use anyhow::{Context as ErrorContext, Error, Result};
use poise::serenity_prelude::all::{CreateBotAuthParameters, Permissions, Scope};
use poise::serenity_prelude::{self as serenity, ApplicationId, Client};

pub struct Data {}
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

pub async fn setup_client(token: &str) -> Result<Client> {
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("{} is connected!", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // let shard_manager = framework.shard_manager().clone();

    let client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .context("Could not log in")?;

    Ok(client)
}

pub fn auth_url(client_id: impl Into<ApplicationId>) -> String {
    CreateBotAuthParameters::new()
        .scopes(&[Scope::Bot, Scope::Guilds, Scope::Identify])
        .permissions(
            Permissions::ADD_REACTIONS
                | Permissions::ATTACH_FILES
                | Permissions::EMBED_LINKS
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::SEND_MESSAGES
                | Permissions::SEND_MESSAGES_IN_THREADS
                | Permissions::VIEW_CHANNEL,
        )
        .client_id(client_id)
        .build()
}
