mod chat;
mod chat_service;
mod error;
mod utils;

use crate::chat::{Chats, User};
use crate::utils::Pagination;

use warp::Filter;

#[tokio::main]
async fn main() {
    let chats = Chats::default();
    let chats = warp::any().map(move || chats.clone());

    let chat = warp::path("chat")
        .and(warp::ws())
        .and(chats)
        .and(warp::path::param())
        .and(warp::query::<User>())
        .and(warp::query::<Pagination>())
        .map(|ws: warp::ws::Ws, chats, chat_name, user: User, page| {
            ws.on_upgrade(move |socket| {
                chat_service::on_user_connection(socket, chats, chat_name, user.username, page)
            })
        });

    warp::serve(chat).run(([127, 0, 0, 1], 3030)).await;
}
