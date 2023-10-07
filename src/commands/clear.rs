use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::MessageFlags;
use serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::prelude::MessageId;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> serenity::Result<()> {
    let options = &cmd.data.options;
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
        .messages(&ctx.http, |retriever| retriever.limit(count))
        .await?;

    // Delete messages
    let message_ids: Vec<MessageId> = messages.iter().map(|message| message.id).collect();
    channel_id.delete_messages(&ctx.http, message_ids).await?;

    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(ChannelMessageWithSource)
            .interaction_response_data(|data| {
                data.flags(MessageFlags::EPHEMERAL)
                    .content(format!("Deleted {} messages.", count))
            })
    })
    .await
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
