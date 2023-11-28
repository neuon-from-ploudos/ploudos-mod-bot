use std::time::Instant;

use color_eyre::eyre::Context;
use poise::{command, CreateReply};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::Ctx;

#[command(slash_command, hide_in_help)]
pub async fn info(ctx: Ctx<'_>) -> color_eyre::Result<()> {
    let user = ctx.framework().bot_id.to_user(&ctx.http()).await?;
    let ping = ctx.ping().await;
    let footer = {
        let footer = CreateEmbedFooter::new(format!("Version: {}", env!("CARGO_PKG_VERSION")));
        if let Some(avatar) = user.avatar_url() {
            footer.icon_url(avatar)
        } else {
            footer
        }
    };
    let resp = 
        CreateReply::default().embed(CreateEmbed::new()
                .title(user.name.to_string())
                .description(format!(
                    r#"
                        >>> **Source**: {}
                        **Uptime**: `{}`
                        **API Ping**: `{}`
                    "#,
                    "[neuon-from-ploudos/ploudos-mod-bot](https://github.com/neuon-from-ploudos/ploudos-mod-bot)",
                    humantime::format_duration(Instant::now() - ctx.data().startup_time),
                    ((ping.as_secs_f32() * 1000.0).round() / 1000.0).to_string() + "s"
                ))
                .footer(
            footer));

    ctx.send(resp)
    .await
    .wrap_err("Failed to respond to command")
    .map(|_| ())
}
