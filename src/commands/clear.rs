use anyhow::Context;
use poise::{command, CreateReply};
use serenity::all::GetMessages;

/// Clear recent messages
#[command(
    slash_command,
    category = "moderation",
    default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn clear(
    ctx: crate::Ctx<'_>,
    #[description = "Number of messages to delete"]
    #[min = 1]
    #[max = 100]
    count: u64,
) -> Result<(), crate::Error> {
    let channel_id = ctx.channel_id();
    let messages = channel_id
        .messages(&ctx.http(), GetMessages::new().limit(count as u8))
        .await?
        .iter()
        .map(|msg| msg.id)
        .collect::<Vec<_>>();
    channel_id.delete_messages(&ctx.http(), messages).await?;

    ctx.send(CreateReply::default().ephemeral(true).content(format!(
        "Deleted {} {}.",
        count,
        if count == 1 { "message" } else { "messages" }
    )))
    .await
    .context("Failed to respond to `clear` command")
    .map(|_| ())
}
