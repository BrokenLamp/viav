use lazy_static::lazy_static;
use serenity::model::channel::ChannelType;
use serenity::model::prelude::GuildChannel;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;
use serenity::utils::Colour;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref ID: Mutex<u8> = Mutex::new(0);
}

pub fn voice_create(
    ctx: &mut Context,
    guild_id: GuildId,
    voice_channel: Arc<RwLock<GuildChannel>>,
    user_id: UserId,
) -> Option<()> {
    let channel_type = ChannelType::Voice;
    guild_id
        .create_channel(&ctx, |c| {
            let mut create_channel = c
                .kind(channel_type)
                .name::<&str>(voice_channel.read().name.as_ref())
                .position((voice_channel.read().position - 1) as u32)
                .permissions(voice_channel.read().permission_overwrites.clone());

            if let Some(category_id) = voice_channel.read().category_id {
                create_channel = create_channel.category(category_id);
            }

            if let Some(user_limit) = voice_channel.read().user_limit {
                create_channel = create_channel.user_limit(user_limit as u32);
            }

            create_channel
        })
        .ok()?;

    let mut lock = ID.lock().unwrap();
    let id = *lock;
    *lock = lock.overflowing_add(1).0;
    std::mem::drop(lock);

    let name = format!("{} / {}", voice_channel.read().name, id);
    voice_channel
        .read()
        .id
        .edit(&ctx, |c| c.name::<&str>(name.as_ref()))
        .ok()?;

    let screen_share_link = format!(
        "https://discordapp.com/channels/{}/{}",
        guild_id.0,
        voice_channel.read().id.0
    );

    let channel_type = ChannelType::Text;
    guild_id
        .create_channel(&ctx, |c| {
            let mut create_channel = c
                .kind(channel_type)
                .topic(format!("**Screen Share: {}** - &{}&{}", screen_share_link, voice_channel.read().id.0, user_id.0))
                .name(format!("voice-viav-{}", id))
                .permissions(voice_channel.read().permission_overwrites.clone());

            if let Some(category_id) = voice_channel.read().category_id {
                create_channel = create_channel.category(category_id);
            }
            create_channel
        })
        .ok()?
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(name)
                        .icon_url("https://cdn.discordapp.com/attachments/451092625894932493/681741191313883186/Viav.png")
                        .url("https://viav.app/")
                })
                .field(
                    "Video",
                    format!("[` Share Screen `]({})", screen_share_link),
                    true,
                )
                .field("Owner", format!("<@{}>", user_id.0), true)
                .colour(Colour::from_rgb(103, 58, 183))
            })
            .reactions(vec!["🔒", "🕵️", "❓"])
        })
        .ok()?
        .pin(&ctx)
        .ok()?;

    Some(())
}
