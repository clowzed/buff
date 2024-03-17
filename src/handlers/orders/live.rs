use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (tx, mut rx) = mpsc::channel(10);
    let (mut sender, _) = socket.split();

    tokio::spawn(async move {
        let connection = state.redis_client().get_async_connection().await.unwrap();
        let mut pubsub = connection.into_pubsub();
        pubsub.subscribe("live_orders").await.unwrap();

        while let Some(msg) = pubsub.on_message().next().await {
            if let Ok(payload) = msg.get_payload() {
                if tx.send(payload).await.is_err() {
                    break;
                }
            }
        }
    });

    while let Some(msg) = rx.recv().await {
        if sender.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}
