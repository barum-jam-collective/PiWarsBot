//! This is the main code for the executable.

#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use dotenv::dotenv;

use serenity::{
    async_trait,
    client::{ClientBuilder, EventHandler},
    framework::standard::{macros::group, StandardFramework},
};
use std::env::var;

use piwarsbot::bot::handlers::ping::PING_COMMAND;

#[allow(missing_docs)]
#[group]
#[commands(ping)]
struct General;

#[allow(missing_docs)]
struct Handler;

#[allow(missing_docs)]
#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = var("DISCORD_TOKEN").unwrap();

    let mut client = ClientBuilder::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client!");

    client.start().await.unwrap();
}
