use super::types::{CommandTypes, TelegramCommand};
use crate::{
    client::Context,
    model::{ChosenInlineResult, InlineQuery, Message, MessageContent, Update, UpdateContent},
};
use std::sync::Arc;

pub struct Framework {
    commands: Vec<TelegramCommand>,
    bot_name: String,
}

impl Framework {
    pub fn new(bot_name: &str) -> Self {
        Self {
            commands: Vec::new(),
            bot_name: bot_name.to_owned(),
        }
    }

    fn match_command(&self, message: &Message, name: &str) -> bool {
        if let MessageContent::Text {
            content, ..
        } = &message.content
        {
            println!("seeing if message matches command, message: \"{}\", command: \"/{}\"", content, name);
            content == &format!("/{}", name)
                || content.starts_with(&format!("/{} ", name))
                || content.starts_with(&format!("/{}@{} ", name, &self.bot_name))
        } else {
            false
        }
    }

    fn fire_message_commands(&self, context: Context, message: Message) {
        for command in &self.commands {
            match command.command.clone() {
                CommandTypes::Default(c) if self.match_command(&message, &command.options.name) => {
                    let ctx = context.clone();
                    let msg = message.clone();
                    println!("calling command {}", &command.options.name);
                    tokio::spawn(async move {c.call(ctx, msg).await });
                },
                _ => (),
            }
        }
    }

//    async fn fire_inline_commands(&self, context: Context, inline: InlineQuery) {
//        let mut collection = Vec::new();
//        for command in &self.commands {
//            match &command.command {
//                CommandTypes::Inline(c) => collection.push(c(context.clone(), inline.clone())),
//                _ => (),
//            }
//        }
//        futures::future::join_all(collection).await;
//    }
//
//    async fn fire_inline_result_commands(&self, context: Context, result: ChosenInlineResult) {
//        let mut collection = Vec::new();
//        for command in &self.commands {
//            match &command.command {
//                CommandTypes::InlineResult(c) => {
//                    collection.push(c(context.clone(), result.clone()))
//                },
//                _ => (),
//            }
//        }
//        futures::future::join_all(collection).await;
//    }

    pub fn add_command(&mut self, command: &TelegramCommand) {
        self.commands.push(command.clone())
    }

    pub fn get_commands(&self) -> &Vec<TelegramCommand> {
        &self.commands
    }

    pub fn fire_commands(fr: Arc<Self>, context: Context, update: Update) {
        match update.content {
                UpdateContent::Message(c) => fr.fire_message_commands(context, c),
//                UpdateContent::InlineQuery(c) => fr.fire_inline_commands(context, c).await,
//                UpdateContent::ChosenInlineResult(c) => {
//                    fr.fire_inline_result_commands(context, c).await
//                },
                _ => (),
            };
    }
}
