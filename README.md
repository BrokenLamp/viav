# Viav

Viav is an open source Discord bot designed to automatically manage your Discord voice channels. When a user joins an empty voice channel, an additional empty channel will be created, so there is always an empty channel for users to join. Additionally, Viav will create a text channel for each voice channel that only the people inside of the channel (as well as administrators) will be able to see.

You can run Viav yourself or add it to your server. Instructions for running it yourself will be added to this readme in the future. You can add it to your server at any time from our [website](https://viav.app/).

Viav is created by Broken Lamp LCC and is licensed under GNU GPLv3.

## Install

To use Viav on your server, follow [this link](https://discord.com/oauth2/authorize?client_id=446151195338473485&permissions=8&scope=bot+applications.commands)

## Self Hosting

To host Viav on your own server

First download Viav from the [releases page](https://github.com/BrokenLamp/viav/releases)

Set the Discord token

```
export DISCORD_TOKEN=abcdefg
```

Run Viav

```
./viav
```

## Building

Make sure you have [Rust](https://rust-lang.org) installed along with Cargo.

```
cargo build
```

This will place a file named `viav` in your `target` folder

## Running Locally

If you'd like to run Viav locally to test on your own Discord build, create a `.evn` file with your `DISCORD_TOKEN` defined inside and use

```
cargo run
```
