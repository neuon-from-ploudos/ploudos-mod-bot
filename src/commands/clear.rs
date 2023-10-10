use poise::command;

use serenity::model::prelude::MessageId;

use crate::State;

/// Clear recent messages
#[command(slash_command)]
pub async fn clear(
    ctx: poise::Context<'_, State, color_eyre::Report>,
    #[description = "Number of messages to delete"]
    #[min = 1]
    #[max = 100]
    count: u64,
) -> color_eyre::Result<()> {
    // Get the channel
    let channel_id = ctx.channel_id();
    let messages = channel_id
        .messages(&ctx.http(), |retriever| retriever.limit(count))
        .await?;

    // Delete messages
    let message_ids: Vec<MessageId> = messages.iter().map(|message| message.id).collect();
    channel_id.delete_messages(&ctx.http(), message_ids).await?;

    ctx.send(|resp| {
        resp.ephemeral(true).content(format!(
            "Deleted {} {}.",
            count,
            if count == 1 { "message" } else { "messages" }
        ))
    })
    .await?;

    Ok(())
}
