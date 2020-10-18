extern crate piwarsbot;
extern crate serenity;

use serenity::async_trait;
use serenity::client::{ClientBuilder, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c|
            c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").unwrap();

    let mut client = ClientBuilder::new(token)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client!");

    if let Err(e) = client.start().await {
        println!("Err: {:?}", e);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Ping pong!").await?;

    Ok(())
}
