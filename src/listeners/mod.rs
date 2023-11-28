// mod link_detection_listener;

use ::serenity::client::FullEvent;
use color_eyre::eyre::Error;
use poise::serenity_prelude as serenity;

use crate::State;

#[allow(unused_variables)]
pub async fn event_listener(
    ctx: &serenity::Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, State, Error>,
    data: &State,
) -> Result<(), Error> {
    // link_detection_listener::link_detection_listener(ctx, event, framework, data).await?;

    Ok(())
}
