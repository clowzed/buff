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
        let mut connection = state.redis_client().get_connection().unwrap();
        let mut pubsub = connection.as_pubsub();
        pubsub.subscribe("live_orders").unwrap();

        while let Ok(msg) = pubsub.get_message() {
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
