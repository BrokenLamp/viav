use serenity::{model::prelude::*, prelude::*, utils::Colour};

pub fn send_help(ctx: &Context, channel_id: ChannelId) -> Option<Message> {
    channel_id.send_message(&ctx, |c| {
        c.embed(|e| {
            e.author(|a| {
                a.name("Viav")
                    .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
                    .url("https://viav.app/")
            })
            .description("**Infinite Voice Channels**\n\n\
                Viav will automatically create voice channels when they're needed, and delete them when they're not.")
            .field("Commands", "Viav doesn't need commands to operate, but for a full list visit our official [**website**](https://viav.app/features/commands).", false)
            .field("Invite", "Viav can be added to any server! Click [**here**](https://discordapp.com/oauth2/authorize?client_id=446151195338473485&permissions=8&scope=bot) to add it to yours!", false)
            .field("Support", "Need help? Join our [**Discord Server**](https://discord.gg/6J27ETD) for some additional support.", false)
            .field("Contribute", "Enjoying Viav? Consider [**donating**](https://donatebot.io/checkout/450361438549311499) or [**voting**](https://top.gg/bot/446151195338473485/vote) for us on top.gg.", false)
            .colour(Colour::from_rgb(103, 58, 183))
        })
    }).ok()
}
