use anyhow::Context;
use poise::{futures_util::StreamExt, CreateReply};

use poise::{command, serenity_prelude::User};
use serenity::{
    all::{Colour, CreateEmbed},
    futures::{self, Stream},
    utils::MessageBuilder,
};

use crate::Ctx;

mod tags {
    include!(concat!(env!("OUT_DIR"), "/tags.rs"));
}

struct Tag<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

/// Print a tag
#[command(slash_command, user_cooldown = 5)]
pub async fn tag(
    ctx: Ctx<'_>,
    #[description = "The tag's id"]
    #[min = 1]
    #[max = 50]
    #[autocomplete = "autocomplete_tag"]
    id: String,
    #[description = " The optional user the tag is meant for"] user: Option<User>,
) -> Result<(), crate::Error> {
    if let Some(tag) = tags::TAGS.get(&(*id)) {
        let embed = CreateEmbed::default()
            .color(Colour::new(0x008060))
            .title(tag.title)
            .description(tag.content);
        let reply = CreateReply::default().embed(embed);
        let reply = if let Some(user) = user {
            reply.content(
                MessageBuilder::new()
                    .push("Hey ")
                    .mention(&user)
                    .push(", please have a look at the following article:")
                    .build(),
            )
        } else {
            reply
        };
        ctx.send(reply).await
    } else {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content(format!("The tag with the id _{}_ does not exist.", &id)),
        )
        .await
    }
    .context("Failed to respond to `tag` command")
    .map(|_| ())
}

async fn autocomplete_tag<'a>(_ctx: Ctx<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(tags::TAGS.keys())
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}
