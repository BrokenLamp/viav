use serenity::{builder::CreateEmbed, model::prelude::*, prelude::*, utils::Colour};

pub async fn send_help(ctx: &Context, channel_id: ChannelId) -> Option<Message> {
    channel_id
        .send_message(&ctx, |c| c.embed(|e| create_help_embed(e)))
        .await
        .ok()
}

pub fn create_help_embed(create_embed: &mut CreateEmbed) -> &mut CreateEmbed {
    create_embed.author(|a| {
        a.name("Viav")
            .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
            .url("https://viav.app/")
    })
    .description("**Infinite Voice Channels**\n\n\
        Viav will automatically create voice channels when they're needed, and delete them when they're not.")
    .field(
        "Commands",
        "-viav help
        -viav controls",
        false
    )
    .field(
        "Helpful Links",
        "[**` Invite `**](https://discordapp.com/oauth2/authorize?client_id=446151195338473485&permissions=8&scope=bot) \
        [**` Website `**](https://viav.app/) \
        [**` Donate `**](https://donatebot.io/checkout/450361438549311499) \
        [**` Top.gg `**](https://top.gg/bot/446151195338473485/vote) \
        [**` Support `**](https://discord.gg/6J27ETD) \
        ",
        false
    )
    .colour(Colour::from_rgb(103, 58, 183))
}
