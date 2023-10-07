use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> serenity::Result<()> {
    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(ChannelMessageWithSource)
            .interaction_response_data(|data| data.content("Hey, I'm alive!"))
    })
    .await
    .map(|_| ())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
