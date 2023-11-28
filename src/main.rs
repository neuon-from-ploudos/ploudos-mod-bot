use std::env;
use std::time::Instant;

use ::serenity::client::ClientBuilder;
use color_eyre::eyre::WrapErr;
use commands::clear;
use commands::info;
use commands::mcstatus;
use commands::ping;
use commands::tag;
use poise::serenity_prelude as serenity;
use poise::Context;
use poise::PrefixFrameworkOptions;
use serenity::prelude::GatewayIntents;

mod commands;
// mod link_validation;
mod listeners;

pub type Ctx<'a> = Context<'a, State, color_eyre::Report>;

pub struct State {
    startup_time: Instant,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
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
        event_handler: |ctx, event, framework, state| {
            Box::pin(listeners::event_listener(ctx, event, framework, state))
        },
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("/".into()),
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State {
                    startup_time: Instant::now(),
                })
            })
        })
        .options(options)
        .build();

    let sm_clone = framework.shard_manager().clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("Shutting down");
        sm_clone.shutdown_all().await;
    });

    ClientBuilder::new(
        env::var("DISCORD_TOKEN")
            .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        intents,
    )
    .framework(framework)
    .await
    .expect("Failed to start the bot")
    .start()
    .await
    .wrap_err("Failed to start the bot")
}
