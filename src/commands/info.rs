use std::time::Instant;

use anyhow::Context;
use poise::{command, CreateReply};
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::Ctx;

#[command(slash_command, hide_in_help)]
pub async fn info(ctx: Ctx<'_>) -> Result<(), crate::Error> {
    let user = ctx.framework().bot_id.to_user(&ctx.http()).await?;
    let ping = ctx.ping().await;
    let ping = ((ping.as_secs_f32() * 1000.0).round() / 1000.0).to_string() + "s";
    let uptime = humantime::format_duration(Instant::now() - ctx.data().startup_time)
        .to_string()
        .split(' ')
        .take(3) // only preserve the 3 most significant units in the formatted duration
        .collect::<Vec<_>>()
        .join(" ");

    let embed = CreateEmbed::default()
            .title(user.name.to_string())
            .description(format!(
                r#"
                    >>> **Source**: {}
                    **Uptime**: `{}`
                    **API Ping**: `{}`
                "#,
                "[neuon-from-ploudos/ploudos-mod-bot](https://github.com/neuon-from-ploudos/ploudos-mod-bot)",
                uptime,
                ping)
            )
            .footer(CreateEmbedFooter::new(format!("Version: {}", env!("CARGO_PKG_VERSION"))));

    ctx.send(CreateReply::default().embed(embed))
        .await
        .context("Failed to respond to `info` command")
        .map(|_| ())
}
