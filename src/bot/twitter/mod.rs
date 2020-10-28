//! This module holds the Twitter handling logic for the bot.

use dotenv::dotenv;
use std::env::var;

use serenity::http::Http;
use serenity::model::id::ChannelId;
use std::sync::Arc;

use egg_mode::{
    stream::{filter, StreamMessage},
    tweet::Tweet,
};
use futures::{executor, TryStreamExt};

/// This function posts the supplied tweet struct to the Discord channel ID provided, using the
/// Http context struct encapsulated in an `Arc<T>`.
async fn post_to_channel(channel_id: u64, ctx: Arc<Http>, tweet: Tweet) {
    ChannelId(channel_id)
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Tweet:");
                e.fields(vec![
                    (
                        "User",
                        format!("@{}", tweet.user.unwrap().screen_name),
                        true,
                    ),
                    ("Contents: {}", tweet.text, true),
                ]);
                e
            });
            m
        })
        .await
        .unwrap();
}

/// This function begins the Twitter stream, that filters on the `#piwars` hashtag.
pub async fn print_feed(ctx: Arc<Http>) {
    dotenv().ok();

    let keypair_consumer = egg_mode::KeyPair::new(
        var("TWITTER_CONSUMER_KEY").unwrap(),
        var("TWITTER_SECRET_KEY").unwrap(),
    );

    let keypair_access = egg_mode::KeyPair::new(
        var("TWITTER_ACCESS_TOKEN").unwrap(),
        var("TWITTER_ACCESS_TOKEN_SECRET").unwrap(),
    );

    let token = egg_mode::Token::Access {
        consumer: keypair_consumer,
        access: keypair_access,
    };

    let stream = filter()
        .track(&["#piwars"])
        .language(&["en"])
        .start(&token)
        .try_for_each(move |m| {
            if let StreamMessage::Tweet(tweet) = m {
                executor::block_on(
                    // this blocks, do we want that?
                    post_to_channel(
                        var("DISCORD_TWITTER_CHANNEL")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap(),
                        ctx.clone(),
                        tweet,
                    ),
                );
            };
            futures::future::ok(())
        });

    if let Err(why) = stream.await {
        println!("Stream error!");
        eprintln!("{:?}", why);
    }
}
