//! This module contains the `ping` handler for the bot.

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[allow(missing_docs)]
#[command]
/// This async function handles the `ping` command for the bot.
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Beep boop beep!").await?;

    Ok(())
}
