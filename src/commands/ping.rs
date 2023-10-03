use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::Context;

pub async fn run(
    _ctx: &Context,
    _options: &[CommandDataOption],
    _cmd: &ApplicationCommandInteraction,
) -> serenity::Result<String> {
    Ok("Hey, I'm alive!".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
