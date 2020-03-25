use core::time::Duration;
use dotenv::dotenv;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use std::env;

mod channel_utils;
mod commands;
mod deck;
mod handler;
mod voice_create;
mod voice_destroy;
mod voice_events;

use commands::GENERAL_GROUP;
use handler::Handler;

pub const MASTER_USER: UserId = UserId(222554302793646083);

fn main() {
    dotenv().ok();

    println!(include_str!("terminal_start.txt"));

    // Login with a bot token from the environment
    let mut client =
        Client::new_with_extras(&env::var("DISCORD_TOKEN").expect("token"), |extras| {
            extras
                .event_handler(Handler)
                .cache_update_timeout(Duration::from_secs(10))
        })
        .expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("-viav "))
            .group(&GENERAL_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
