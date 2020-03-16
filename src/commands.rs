use super::deck;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use serenity::utils::Colour;

#[group]
#[commands(ping, help, controls)]
struct General;

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

#[command]
fn controls(ctx: &mut Context, msg: &Message) -> CommandResult {
    controls_command(&ctx, msg);
    Ok(())
}

fn controls_command(ctx: &Context, msg: &Message) -> Option<()> {
    let channel_lock = msg.channel(ctx)?.guild()?;
    let channel = &*channel_lock.read();
    let topic = channel.topic.clone()?;

    let screen_share_link = {
        let start_bytes = topic.find("Share: ")? + 7;
        let end_bytes = topic.find(" - &")?;
        &topic[start_bytes..end_bytes]
    };

    let user_id = {
        let mut split = topic.split("&");
        split.next()?;
        split.next()?;
        UserId(split.next()?.parse::<u64>().ok()?)
    };

    deck::create_deck(
        ctx,
        channel,
        "Viav Controls".into(),
        screen_share_link.into(),
        user_id,
    );
    Some(())
}
