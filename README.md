[![Crate](https://img.shields.io/crates/v/telexide?style=flat-square)](https://crates.io/crates/telexide)
[![Docs](https://docs.rs/telexide/badge.svg)](https://docs.rs/telexide)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fbaev1%2Ftelexide%2Fbadge&style=flat-square)](https://actions-badge.atrox.dev/baev1/telexide/goto)
[![Rust Version](https://img.shields.io/badge/rust-1.41.0+-93450a.svg?style=flat-square)](https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html)

# telexide

Telexide is an easy to use library for making a telegram bot, built on tokio and hyper.

View the [examples] on how to make and structure a bot.



## Example Bot

A basic ping-pong bot can be written like:

```rust
use std::env;
use telexide::{api::types::SendMessage, prelude::*};

#[command(description = "just a ping-pong command")]
async fn ping(context: Context, message: Message) -> CommandResult {
    context
        .api
        .send_message(SendMessage::new(message.chat.get_id(), "pong"))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");

    ClientBuilder::with_framework(
        create_framework!(&bot_name, ping),
        token
    )
        .start()
        .await
}
```

For more examples, please see the examples dir.

## Features

- [X] easy to use and customisable client
- [X] long-polling based update handling
    - [X] set your own timeout
    - [X] set your own limit for updates gotten at once
- [X] easy to use, macro-based command framework
- [X] easy to use and heavily customisable api client
    - [X] use your own hyper client
    - [X] use your own api struct so you control the get and post methods
    - [X] includes all telegram api endpoints

#### Planned:
- [ ] webhook based update handling
- [ ] choosing what you want from the lib sing feature flags
- [ ] subscribe to non-message events using command (or similar) framework
    - [ ] run command on receiving an inline query or answer to one
    - [ ] run command on receiving a poll that matches your requirements
- [ ] wait_for style Context method to wait for an update matching your criteria
- [ ] more builder methods for creating api data
- [ ] methods on models for easier calling of API endpoints (like `ChatMember::kick`)v

## Installation

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
telexide = "0.1"
```

[examples]: https://github.com/Baev1/telexide/blob/master/examples