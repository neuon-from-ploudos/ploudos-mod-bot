mod link_detection_listener;

use color_eyre::eyre::Error;
use poise::{serenity_prelude as serenity, Event};

use crate::State;

pub async fn event_listener(
    ctx: &serenity::Context,
    event: &Event<'_>,
    framework: poise::FrameworkContext<'_, State, Error>,
    data: &State,
) -> Result<(), Error> {
    link_detection_listener::link_detection_listener(ctx, event, framework, data).await?;

    Ok(())
}
