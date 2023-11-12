use crate::State;

/// The ping command
#[poise::command(prefix_command)]
pub async fn mcstatus(
    ctx: poise::Context<'_, State, color_eyre::Report>,
) -> color_eyre::Result<()> {
    ctx.say(
        r#"
Ayo, what you think you're doing with this command?
PloudOS is **DEAD** so you can't have any info bruh
"mCsTaTuS" what...
Have a nice day :slight_smile:
        "#,
    )
    .await?;
    Ok(())
}
