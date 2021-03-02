extern crate log;
extern crate pretty_env_logger;

use core::time::Duration;
use dotenv::dotenv;
use serenity::client::{bridge::gateway::GatewayIntents, Client};
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use std::env;

mod channel_utils;
mod commandable_ops;
mod commands;
mod deck;
mod error_display;
mod handler;
mod help;
mod models;
mod slash_commands;
mod voice_create;
mod voice_destroy;
mod voice_events;

use commands::GENERAL_GROUP;
use handler::Handler;

pub const MASTER_USER: UserId = UserId(222554302793646083);
pub const THREADS: usize = 512;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    println!(include_str!("terminal_start.txt"));

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-viav "))
        .group(&GENERAL_GROUP);

    let token = &env::var("DISCORD_TOKEN").expect("token");

    let num_shards = env::var("NUM_SHARDS")
        .ok()
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(1u64);

    let intents: GatewayIntents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_INTEGRATIONS
        | GatewayIntents::GUILD_EMOJIS;

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .cache_update_timeout(Duration::from_secs(10))
        .intents(intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Shards: {}", num_shards);

    if let Err(why) = client.start_shards(num_shards).await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
