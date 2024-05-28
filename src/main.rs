use std::time::Instant;

use anyhow::Context as _;
use commands::clear;
use commands::info;
use commands::mcstatus;
use commands::ping;
use commands::tag;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use poise::Context;
use poise::PrefixFrameworkOptions;

mod commands;

type Error = anyhow::Error;
pub type Ctx<'a> = Context<'a, State, Error>;

pub struct State {
    startup_time: Instant,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let discord_token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let options = poise::FrameworkOptions {
        commands: vec![
            ping::ping(),
            clear::clear(),
            tag::tag(),
            info::info(),
            mcstatus::mcstatus(),
        ],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("/".into()),
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State {
                    startup_time: Instant::now(),
                })
            })
        })
        .options(options)
        .build();

    let client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
