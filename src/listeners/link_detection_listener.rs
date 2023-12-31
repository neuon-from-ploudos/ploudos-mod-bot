use std::collections::HashMap;
use std::collections::HashSet;

use color_eyre::eyre::Error;
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;
use poise::Event;
use regex::Regex;

use crate::link_validation::link_detection::is_url_bad;
use crate::link_validation::link_detection::setup;
use crate::State;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https?:\/\/.*?[ $\/]").unwrap());
static LINK_DETECT: Lazy<(HashSet<std::string::String>, HashMap<char, char>)> = Lazy::new(setup);

pub async fn link_detection_listener(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, State, Error>,
    _data: &State,
) -> Result<(), Error> {
    if let Event::Message { new_message } = event {
        let (tks, char_map) = (&LINK_DETECT.0, &LINK_DETECT.1);

        let text = &new_message.content;

        let urls: Vec<&str> = URL_REGEX.find_iter(text).map(|m| m.as_str()).collect();

        if !urls.is_empty() && !new_message.is_own(ctx) {
            let urls: Vec<(&str, bool)> = urls
                .iter()
                .map(|&url| (url, is_url_bad(char_map, tks, url)))
                .collect();
            new_message.reply(ctx, format!("URLS: {:#?}", urls)).await?;
        }
    }
    Ok(())
}
