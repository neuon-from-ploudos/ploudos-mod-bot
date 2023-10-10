use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::ApplicationCommandInteraction, command::CommandOptionType, UserId,
    },
    prelude::Context,
};
use time::PrimitiveDateTime;

struct Tag {
    name: String,
    author: UserId,
    content: String,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
}

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> serenity::Result<()> {
    let options = &cmd.data.options;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("faq")
        .description("Frequently asked questions")
        .create_option(|option| {
            option
                .name("create")
                .description("Create a new faq entry")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|opt| {
                    opt.name("tag name")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|opt| {
                    opt.name("content")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
}
