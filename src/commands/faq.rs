use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction, prelude::Context,
};

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> serenity::Result<()> {
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("faq")
        .description("Frequently asked questions")
}
