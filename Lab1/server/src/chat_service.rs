use crate::chat::Chats;
use crate::utils::Pagination;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

pub async fn on_user_connection(
    ws: WebSocket,
    chats: Chats,
    chat_name: String,
    username: String,
    page: Pagination,
) {
    let (mut user_ws_tx, user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });
    chats
        .join(chat_name.clone(), username, page, user_ws_rx, tx)
        .await;
}
