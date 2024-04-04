use crate::{state::AppState, Order};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use entity::{
    order::{Column as OrderColumn, Entity as OrderEntity},
    sea_orm_active_enums::Status,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
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

    let orders_to_send = OrderEntity::find()
        .filter(OrderColumn::Status.eq(Status::Succeeded))
        .order_by_desc(OrderColumn::FinishedAt)
        .limit(10)
        .all(state.database_connection())
        .await
        .unwrap_or_default();

    tokio::spawn(async move {
        let connection = state.redis_client().get_async_connection().await.unwrap();
        let mut pubsub = connection.into_pubsub();
        pubsub.subscribe("live_orders").await.unwrap();

        for order in orders_to_send {
            tx.send(serde_json::to_string(&Into::<Order>::into(order)).unwrap_or_default())
                .await
                .ok();
        }

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
