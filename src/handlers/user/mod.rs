use crate::{
    errors::AppError,
    extractors::user_jwt::AuthJWT,
    services::{
        chat::{GetChatParameters, SendMessageParameters, Sender, Service as ChatService},
        users::Service as UsersService,
    },
    state::AppState,
    ChatHistory, ChatResponse, GetChatRequest, Message, SendMessageResponse, UploadData,
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use axum_typed_multipart::TypedMultipart;
use chrono::NaiveDateTime;
use entity::{
    chat::Entity as ChatEntity, image::Entity as ImageEntity, message::Entity as MessageEntity,
    user::Model as UserModel,
};
use redis::AsyncCommands;
use sea_orm::{prelude::Decimal, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_util::io::ReaderStream;
use utoipa::{IntoParams, ToSchema};

#[derive(serde::Serialize, serde::Deserialize, ToSchema, Debug)]
pub struct User {
    pub steam_id: i64,
    pub trade_url: Option<String>,
    pub email: Option<String>,
    pub registered_at: NaiveDateTime,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        Self {
            steam_id: value.steam_id,
            trade_url: value.trade_url,
            email: value.email,
            registered_at: value.registered_at,
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/user",
    responses(
        (status = 200, description = "User was successfully retrieved",    body = User),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn get_user(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
) -> axum::response::Response {
    match UsersService::get_by_steam_id(user.steam_id, app_state.database_connection()).await {
        Ok(Some(user)) => Json(Into::<User>::into(user)).into_response(),
        Ok(None) => AppError::UserWasNotFound(user.steam_id).into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TradeUrlForm {
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct EmailForm {
    email: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/api/user/email",
    responses(
        (status = 200, description = "Email was successfully changed"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    request_body = EmailForm,
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_email(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Json(payload): Json<EmailForm>,
) -> axum::response::Response {
    match UsersService::set_email(
        user.steam_id,
        payload.email,
        app_state.database_connection(),
    )
    .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/user/trade-url",
    responses(
        (status = 200, description = "Trade url was successfully changed"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    request_body = TradeUrlForm,
    security(
        ("jwt_user" = [])
    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn set_trade_url(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Json(payload): Json<TradeUrlForm>,
) -> axum::response::Response {
    match UsersService::set_trade_url(user.steam_id, payload.url, app_state.database_connection())
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct TopUser {
    pub steam_id: i64,
    #[schema(value_type = String)]
    pub amount: Decimal,
}

impl From<crate::services::users::TopUser> for TopUser {
    fn from(value: crate::services::users::TopUser) -> Self {
        Self {
            steam_id: value.steam_id,
            amount: value.amount,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, ToSchema, Debug, IntoParams)]
pub struct Bounds {
    limit: u64,
    offset: u64,
}

#[utoipa::path(
    get,
    path = "/api/user/top",
    responses(
        (status = 200, description = "Top users were successfully retrieved", body = [TopUser]),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    params(
        Bounds

    )
)]
#[tracing::instrument(skip(app_state))]
pub async fn get_top(
    State(app_state): State<Arc<AppState>>,
    Query(payload): Query<Bounds>,
) -> axum::response::Response {
    match UsersService::top(
        payload.limit,
        payload.offset,
        app_state.database_connection(),
    )
    .await
    {
        Ok(users) => Json(
            users
                .into_iter()
                .map(Into::<TopUser>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/user/chat",
    request_body = GetChatRequest,
    responses(
        (status = 200, description = "Chat was successfully retrieved",    body = ChatResponse),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
    ),
    security(
        ("jwt_user" = [])
    )
)]
pub async fn chat(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Json(payload): Json<GetChatRequest>,
) -> Response {
    let moderator_id: i64 = match payload.id.parse() {
        Ok(id) => id,
        Err(cause) => return Into::<AppError>::into(cause).into_response(),
    };

    let params = GetChatParameters {
        moderator_id: moderator_id,
        steam_id: user.steam_id,
    };

    match ChatService::chat(params, app_state.database_connection()).await {
        Ok(chat) => Json(Into::<ChatResponse>::into(chat)).into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/user/chat/{id}/message",
    request_body = UploadData,
    params(("id" = i64, Path, description = "Chat id")),

    responses(
        (status = 200, description = "Message was successfully sent",    body = SendMessageResponse),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
        (status = 403, description = "User is not a member of this chat"),
        (status = 404, description = "Chat  was not found"),

    ),
    security(
        ("jwt_user" = [])
    )
)]
pub async fn send_message(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Path(chat_id): Path<i64>,
    TypedMultipart(UploadData { images, text }): TypedMultipart<UploadData>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(connection) => {
            let _ = match ChatEntity::find_by_id(chat_id).one(&connection).await {
                Ok(Some(chat)) => match chat.steam_id == user.steam_id {
                    true => chat,
                    false => return StatusCode::FORBIDDEN.into_response(),
                },
                Ok(None) => return StatusCode::NOT_FOUND.into_response(),
                Err(_cause) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            let params = SendMessageParameters {
                folder: app_state.configuration().upload_folder().clone(),
                chat_id,
                sender: Sender::User,
                text,
                images: &images,
            };

            match ChatService::send_message(params, &connection).await {
                Ok(res) => {
                    if let Err(cause) = connection.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }

                    let send = SendMessageResponse {
                        message: Into::<Message>::into(res.0),
                        images_ids: res.1,
                    };

                    match app_state.redis_client().get_async_connection().await {
                        Ok(mut connection) => {
                            let _: Result<(), _> = connection
                                .publish(
                                    format!("chat-{}", chat_id),
                                    serde_json::to_string(&send).unwrap(),
                                )
                                .await;
                        }
                        Err(cause) => {
                            tracing::warn!(%cause, "Failed to connect to redis!");
                        }
                    };

                    Json(send).into_response()
                }
                Err(cause) => Into::<AppError>::into(cause).into_response(),
            }
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/user/chat/{id}/history",
    params(("id" = i64, Path, description = "Chat id")),

    responses(
        (status = 200, description = "History was successfully retrieved", body = ChatHistory),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
        (status = 403, description = "User is not a member of this chat"),
        (status = 404, description = "Chat was not found"),

    ),
    security(
        ("jwt_user" = [])
    )
)]
pub async fn history(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Path(chat_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(connection) => {
            let chat = match ChatEntity::find_by_id(chat_id).one(&connection).await {
                Ok(Some(chat)) => match chat.steam_id == user.steam_id {
                    true => chat,
                    false => return StatusCode::FORBIDDEN.into_response(),
                },
                Ok(None) => return StatusCode::NOT_FOUND.into_response(),
                Err(_cause) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            let resp = match ChatService::history(chat.id, &connection).await {
                Ok(res) => Json(Into::<ChatHistory>::into(res)).into_response(),
                Err(cause) => return Into::<AppError>::into(cause).into_response(),
            };

            if let Err(cause) = connection.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }

            resp
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/user/chat/{id}/image/{id}",
    params(("id" = (i64, i64), Path, description = "Chat id and image id")),

    responses(
        (status = 200, description = "Image was successfully retrieved"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
        (status = 403, description = "User is not a member of this chat"),
        (status = 404, description = "Chat was not found"),

    ),
    security(
        ("jwt_user" = [])
    )
)]
pub async fn image(
    State(app_state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Path((chat_id, image_id)): Path<(i64, i64)>,
) -> Response {
    let _chat = match ChatEntity::find_by_id(chat_id)
        .one(app_state.database_connection())
        .await
    {
        Ok(Some(chat)) => match chat.steam_id == user.steam_id {
            true => chat,
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_cause) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let image = match ImageEntity::find_by_id(image_id)
        .one(app_state.database_connection())
        .await
    {
        Ok(Some(image)) => image,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(cause) => return AppError::InternalServerError(Box::new(cause)).into_response(),
    };

    let message = match MessageEntity::find_by_id(image.message_id)
        .one(app_state.database_connection())
        .await
    {
        Ok(Some(message)) => message,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(cause) => return AppError::InternalServerError(Box::new(cause)).into_response(),
    };

    if message.chat_id != chat_id {
        return StatusCode::FORBIDDEN.into_response();
    }

    Body::from_stream(ReaderStream::new(
        match tokio::fs::File::open(&image.path).await {
            Ok(file) => file,
            Err(_cause) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    ))
    .into_response()
}

use axum::extract::{
    ws::{Message as WsMessage, WebSocket},
    WebSocketUpgrade,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    AuthJWT(user): AuthJWT,
    Path(chat_id): Path<i64>,
) -> Response {
    match ChatEntity::find_by_id(chat_id)
        .one(state.database_connection())
        .await
    {
        Ok(Some(chat)) => {
            if chat.steam_id != user.steam_id {
                return StatusCode::FORBIDDEN.into_response();
            }
        }
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, chat_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, chat_id: i64) {
    let (tx, mut rx) = mpsc::channel(10);
    let (mut sender, _) = socket.split();

    tokio::spawn(async move {
        let mut connection = state.redis_client().get_connection().unwrap();
        let mut pubsub = connection.as_pubsub();
        pubsub.subscribe(format!("chat-{}", chat_id)).unwrap();

        while let Ok(msg) = pubsub.get_message() {
            if let Ok(payload) = msg.get_payload() {
                if tx.send(payload).await.is_err() {
                    break;
                }
            }
        }
    });

    // Sends Order component
    while let Some(msg) = rx.recv().await {
        if sender.send(WsMessage::Text(msg)).await.is_err() {
            break;
        }
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user))
        .route("/trade-url", patch(set_trade_url))
        .route("/email", patch(set_email))
        .route("/top", get(get_top))
        .route("/chat", patch(chat))
        .route("/chat/:id/message", post(send_message))
        .route("/chat/:id/history", get(history))
        .route("/chat/:id", get(websocket_handler))
        .route("/chat/:id/image/:id", get(image))
}
