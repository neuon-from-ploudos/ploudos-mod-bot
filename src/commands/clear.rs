use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::MessageId;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::prelude::Context;

pub async fn run(
    ctx: &Context,
    options: &[CommandDataOption],
    cmd: &ApplicationCommandInteraction,
) -> serenity::Result<String> {
    let count = options
        .get(0)
        .expect("Number of messages to delete") // todo: expect is gay
        .resolved
        .as_ref()
        .expect("IDK"); // todo: expect is gay
    let count = match count {
        CommandDataOptionValue::Integer(n) => *n as u64, // todo: check for <0
        _ => todo!(),
    };

    // Fetch the channel
    let channel_id = cmd.channel_id;
    let messages = channel_id
        .messages(&ctx.http, |retriever| {
            retriever.limit(count + 1)
        }) // +1 to include the invoking command
        .await?;

    // Delete messages
    let message_ids: Vec<MessageId> = messages.iter().map(|message| message.id).collect();
    channel_id.delete_messages(&ctx.http, message_ids).await?;

    Ok("Messages deleted".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("clear")
        .description("Clear n messages")
        .create_option(|opt| {
            opt.name("messages")
                .description("Number of messages to delete")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(100)
                .required(true)
        })
}
