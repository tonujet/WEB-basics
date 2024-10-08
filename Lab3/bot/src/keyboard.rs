use crate::error::AppError;
use crate::State;
use std::error::Error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

pub enum KeyboardButton {
    Student { message: &'static str },
    Contacts { message: &'static str },
    Technologies { message: &'static str },

    Back,
    UNKNOWN,
}

impl KeyboardButton {
    pub async fn press(
        &self,
        bot: Bot,
        q: CallbackQuery,
        state: State,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let msg = q.message.ok_or(AppError::Bot(
            "There isn't message in CallbackQuery".to_string(),
        ))?;

        match self {
            KeyboardButton::Student { message }
            | KeyboardButton::Contacts { message }
            | KeyboardButton::Technologies { message } => {
                bot.edit_message_text(msg.chat.id, msg.id, *message)
                    .reply_markup(back_keyboard())
                    .await?;
            }

            KeyboardButton::Back => {
                bot.edit_message_text(
                    msg.chat.id,
                    msg.id,
                    "🤝 Welcome \n🟢 Choose one of the following options \n🔵 Write a message to Groq LLM🤖(It should not started with '/')",
                )
                .reply_markup(menu_keyboard())
                .await?;
                state
                    .last_menu_reply
                    .write()
                    .await
                    .insert(msg.from().unwrap().id, msg.id);
            }

            KeyboardButton::UNKNOWN => {
                log::error!("Unknown button was pressed");
                bot.send_message(
                    msg.chat.id,
                    "Welcome ^^ \nChoose one of the following commands",
                )
                .reply_markup(menu_keyboard())
                .await?;
            }
        }

        Ok(())
    }
}

impl From<&str> for KeyboardButton {
    fn from(value: &str) -> Self {
        let name = value.to_lowercase();
        match name.as_str() {
            "student" => Self::Student { message: "Vorobei Anton IA-11" },
            "it-technologies" => Self::Technologies {
                message: "Embedded/System/Blockchain software developer",
            },
            "contacts" => Self::Contacts {
                message: "Email: example@example.com\nPhone: +380505050505",
            },

            "back" => Self::Back,
            _ => {
                log::warn!("Selected unknown keyboard button");
                Self::UNKNOWN
            }
        }
    }
}

impl From<KeyboardButton> for &str {
    fn from(button: KeyboardButton) -> Self {
        match button {
            KeyboardButton::Student { .. } => "Student",
            KeyboardButton::Technologies { .. } => "IT-Technologies",
            KeyboardButton::Contacts { .. } => "Contacts",

            KeyboardButton::Back => "Back",
            KeyboardButton::UNKNOWN => {
                log::warn!("Unknown keyboard button converted to string");
                "unknown"
            }
        }
    }
}

pub fn menu_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let choices = [
        KeyboardButton::from("student"),
        KeyboardButton::from("it-technologies"),
        KeyboardButton::from("contacts"),
    ];

    for choice in choices {
        let choice: &str = choice.into();
        let row = vec![InlineKeyboardButton::callback(
            choice.to_owned(),
            choice.to_owned(),
        )];
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn back_keyboard() -> InlineKeyboardMarkup {
    let text: &str = KeyboardButton::Back.into();
    let button = InlineKeyboardButton::callback(text.to_owned(), text.to_owned());
    let keyboard = vec![vec![button]];
    InlineKeyboardMarkup::new(keyboard)
}
