use poise::futures_util::StreamExt;
use std::collections::HashMap;

use color_eyre::eyre::Context;
use poise::{command, serenity_prelude::User};
use serenity::{
    futures::{self, Stream},
    utils::{Colour, MessageBuilder},
};

use crate::Ctx;

lazy_static! {
    static ref TAGS: HashMap<&'static str, Tag<'static>> = {
        let mut map = HashMap::new();
        let tags = vec![Tag {
            id: "ploudos-closed",
            title: "Why did PloudOS close?",
            content: "",
        }];
        for tag in tags {
            map.insert(tag.id, tag);
        }
        map
    };
}

struct Tag<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
}

/// Print a tag
#[command(slash_command)]
pub async fn tag(
    ctx: Ctx<'_>,
    #[description = "The tag's id"]
    #[min = 1]
    #[max = 50]
    #[autocomplete = "autocomplete_tag"]
    id: String,
    #[description = " The optional user the tag is meant for"] user: Option<User>,
) -> color_eyre::Result<()> {
    if let Some(tag) = TAGS.get(&(*id)) {
        ctx.send(|resp| {
            resp.embed(|embed| {
                embed
                    .color(Colour::new(0x008060))
                    .title(tag.title)
                    .description(tag.content)
            });
            if let Some(user) = user {
                resp.content(
                    MessageBuilder::new()
                        .push("Hey ")
                        .mention(&user)
                        .push(", please have a look at the following article:")
                        .build(),
                );
            };
            resp
        })
        .await
    } else {
        ctx.send(|resp| {
            resp.ephemeral(true)
                .content(format!("The tag with the id _{}_ does not exist.", &id))
        })
        .await
    }
    .wrap_err("Failed to respond to command")
    .map(|_| ())
}

async fn autocomplete_tag<'a>(_ctx: Ctx<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(TAGS.keys())
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}
