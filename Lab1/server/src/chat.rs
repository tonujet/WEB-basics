use crate::error::ChatError;
use crate::utils::Pagination;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};

#[derive(Debug, Clone)]
struct Chat {
    name: Arc<String>,
    users: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    messages: Arc<RwLock<Vec<ChatMessage>>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IncomingMessage {
    text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMessage {
    username: String,
    text: String,
}

impl ChatMessage {
    pub fn new(username: String, text: String) -> Self {
        Self { username, text }
    }
}

impl Chat {
    pub fn new(name: String) -> Self {
        Chat {
            name: Arc::new(name),
            users: Default::default(),
            messages: Default::default(),
        }
    }

    pub async fn disconnect_user(&self, username: &String) {
        eprintln!("good bye user: {}", username);
        self.users.write().await.remove(username);
    }

    pub async fn get_start_messages(&self, _page: Pagination, tx: mpsc::UnboundedSender<Message>) {
        let messages = self.messages.read().await;
        let all_messages = serde_json::to_string(&*messages).unwrap();
        let a = tx.send(Message::text(all_messages));
        if a.is_err() {
            println!("{}", a.err().unwrap())
        }
    }

    pub async fn send_messages(
        &self,
        username: &String,
        mut user_ws_rx: SplitStream<WebSocket>,
    ) -> Result<(), ChatError> {
        while let Some(result) = user_ws_rx.next().await {
            let mess = receive_message(result)?;
            let IncomingMessage { text } = serde_json::from_str::<IncomingMessage>(&mess)?;
            let message = ChatMessage::new(username.clone(), text.clone().parse().unwrap());
            self.messages.write().await.insert(0, message.clone());

            for (other_username, tx) in self.users.read().await.iter() {
                if username != other_username {
                    let msg = serde_json::to_string(&vec![message.clone()])?;
                    if let Err(_disconnected) = tx.send(Message::text(msg)) {
                        // The tx is disconnected
                    }
                }
            }
        }
        Ok(())
    }
}

fn receive_message(message: Result<Message, warp::Error>) -> Result<String, ChatError> {
    let msg = message?;
    let msg = msg.to_str().map_err(|_| ChatError::InvalidMessage())?;
    Ok(msg.to_string())
}

#[derive(Default, Clone)]
pub struct Chats {
    chats: Arc<RwLock<HashMap<String, Chat>>>,
}

impl Chats {
    pub async fn join(
        &self,
        req_chat_name: String,
        username: String,
        page: Pagination,
        user_ws_rx: SplitStream<WebSocket>,
        tx: mpsc::UnboundedSender<Message>,
    ) {
        let chat = self
            .insert_chat(req_chat_name, username.clone(), tx.clone())
            .await;
        match chat {
            Ok(chat) => {
                chat.get_start_messages(page, tx.clone()).await;
                let res = chat.send_messages(&username, user_ws_rx).await;
                if res.is_err() {
                    let message = res.err().unwrap().to_request_body();
                    let _ = tx.send(message);
                }
                chat.disconnect_user(&username).await;
            }
            Err(error) => {
                let _ = tx.send(error.to_request_body());
            }
        }
    }

    async fn insert_chat(
        &self,
        chat_name: String,
        username: String,
        tx: mpsc::UnboundedSender<Message>,
    ) -> Result<Chat, ChatError> {
        let mut chats = self.chats.write().await;

        match chats.get(&chat_name) {
            Some(chat) => {
                let mut users = chat.users.write().await;
                if users.insert(username.clone(), tx.clone()).is_some() {
                    Err(ChatError::UsernameTaken(chat_name))?
                }
                Ok(chat.clone())
            }
            None => {
                let chat = Chat::new(chat_name.clone());
                chat.users
                    .write()
                    .await
                    .insert(username.clone(), tx.clone());
                chats.insert(chat_name, chat.clone());
                Ok(chat)
            }
        }
    }
}
