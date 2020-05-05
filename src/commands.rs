use super::deck;
use super::help;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;

#[group]
#[commands(ping, help, info, controls)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    help::send_help(ctx, msg.channel_id).await;

    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |c| c.content(format!("Shard ID: {}", ctx.shard_id)))
        .await?;
    Ok(())
}

#[command]
async fn controls(ctx: &Context, msg: &Message) -> CommandResult {
    match controls_command(&ctx, msg).await {
        Some(_) => Ok(()),
        None => {
            msg.reply(ctx, "Error").await?;
            Ok(())
        }
    }
}

async fn controls_command(ctx: &Context, msg: &Message) -> Option<Message> {
    let channel_id = msg.channel_id;
    let topic = msg.channel(ctx).await?.guild()?.topic?.clone();

    let user_id = {
        let mut split = topic.split("&");
        split.next()?;
        split.next()?;
        UserId(split.next()?.parse::<u64>().ok()?)
    };

    deck::create_deck(ctx, channel_id, "Viav Controls".into(), user_id).await
}
