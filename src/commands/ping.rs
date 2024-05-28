#[poise::command(slash_command)]
pub async fn ping(ctx: crate::Ctx<'_>) -> Result<(), crate::Error> {
    ctx.say("Hey, I'm alive!").await?;
    Ok(())
}
