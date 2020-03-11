use maplit::hashmap;
use serenity::{
    model::prelude::{GuildChannel, Reaction, User},
    prelude::Context,
};
use std::collections::HashMap;

struct Action {
    on_add: &'static Fn(u64) -> u64,
    on_remove: &'static Fn(u64) -> u64,
}

pub static ACTIONS: HashMap<u64, &'static Action> = make_hashmap();

const fn make_hashmap() -> HashMap<u64, &'static Action> {
    hashmap! {
        684471911920566281u64 => &Action {
            on_add: &|x: u64| x,
            on_remove: &|x: u64| x,
        },
    }
}
