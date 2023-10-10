use crate::State;

/// The ping command
#[poise::command(slash_command)]
pub async fn ping<'a>(
    ctx: poise::Context<'a, State, color_eyre::Report>,
) -> color_eyre::Result<()> {
    ctx.say("Hey, I'm alive!").await?;
    Ok(())
}
