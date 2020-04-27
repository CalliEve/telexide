use super::types::{CommandTypes, TelegramCommand};
use crate::{
    client::Context,
    model::{Message, MessageContent, MessageEntity, Update, UpdateContent},
};

/// A utility for easily managing commands.
///
/// Refer to the [module-level documentation](index.html) for more detail
pub struct Framework {
    commands: Vec<TelegramCommand>,
    bot_name: String,
}

impl Framework {
    /// Creates a new framework instance given the bot name
    pub fn new(bot_name: &str) -> Self {
        Self {
            commands: Vec::new(),
            bot_name: bot_name.to_owned(),
        }
    }

    fn match_command(&self, message: &Message, name: &str) -> bool {
        if let MessageContent::Text {
            entities,
            content,
        } = &message.content
        {
            for entity in entities {
                if let MessageEntity::BotCommand(ref t) = entity {
                    let t = t.get_text(content);
                    return t == format!("/{}", name)
                        || t == format!("/{}@{} ", name, &self.bot_name);
                }
            }
        }
        false
    }

    #[allow(clippy::needless_pass_by_value)]
    fn fire_message_commands(&self, context: Context, message: Message) {
        for command in &self.commands {
            match command.command.clone() {
                CommandTypes::Default(c) if self.match_command(&message, &command.options.name) => {
                    let ctx = context.clone();
                    let msg = message.clone();
                    println!("calling command {}", &command.options.name);
                    tokio::spawn(async move { c.call(ctx, msg).await });
                },
                _ => (),
            }
        }
    }

    /// add a command to the registered commands
    pub fn add_command(&mut self, command: &TelegramCommand) {
        self.commands.push(command.clone())
    }

    /// get all registered commands
    pub fn get_commands(&self) -> &Vec<TelegramCommand> {
        &self.commands
    }

    /// fires off all commands matching the content in the update
    pub fn fire_commands(&self, context: Context, update: Update) {
        if let UpdateContent::Message(c) = update.content {
            self.fire_message_commands(context, c);
        }
    }
}
