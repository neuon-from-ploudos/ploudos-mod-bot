use std::env;

use color_eyre::eyre::WrapErr;
use commands::clear;
use commands::ping;
use poise::serenity_prelude as serenity;
use poise::Event;
use serenity::prelude::GatewayIntents;

mod commands;

pub struct State;

struct Handler;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let options = poise::FrameworkOptions {
        commands: vec![ping::ping(), clear::clear()],
        event_handler: |_ctx, event, _framework, _state| {
            Box::pin(event_handler(_ctx, event, _framework, _state))
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            env::var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State)
            })
        })
        .options(options)
        .intents(intents)
        .run()
        .await
        .wrap_err("Failed to start the bot")
}

async fn event_handler(
    _ctx: &serenity::Context,
    _event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, State, color_eyre::Report>,
    _data: &State,
) -> color_eyre::Result<()> {
    Ok(())
}
