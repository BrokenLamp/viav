use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::Colour;
use std::env;

mod handler;
mod voice_create;
mod voice_destroy;
mod voice_events;

use handler::Handler;

#[group]
#[commands(ping)]
#[commands(help)]
struct General;

fn main() {
    println!(include_str!("terminal_start.txt"));

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("-viav ")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(ctx, |c| {
        c.embed(|e| {
            e.author(|a| {
                a.name("Viav")
                    .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
                    .url("https://viav.app/")
            })
            .description(include_str!("help.md"))
            .colour(Colour::from_rgb(103, 58, 183))
        })
    })?;

    Ok(())
}
