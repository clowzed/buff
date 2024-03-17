use crate::{
    extractors::admin_jwt::ModeratorAuthJWT,
    services::{
        admin::moderators::{
            AssignModeratorParameters, CreateModeratorParameters, Service as AdminService,
            UnassignModeratorParameters,
        },
        auth::{JwtCheckParams, ResetPasswordParameters, Service as AuthService},
        chat::{GetChatParameters, SendMessageParameters, Sender, Service as ChatService},
    },
    Order,
};
use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::NaiveDateTime as DateTime;
use entity::{
    admin::{Entity as AdminEntity, Model as AdminModel},
    chat::{Entity as ChatEntity, Model as ChatModel},
    image::{Entity as ImageEntity, Model as ImageModel},
    message::{Entity as MessageEntity, Model as MessageModel},
};

use redis::AsyncCommands;
use sea_orm::{EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;

use crate::{errors::AppError, extractors::admin_jwt::AdminAuthJWT, state::AppState};

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ModeratorCredentials {
    pub login: String,
    pub password: String,
}

#[derive(TryFromMultipart, ToSchema)]
pub struct UploadData {
    #[schema(value_type = Option<String>, format = Binary)]
    pub image: Option<FieldData<Bytes>>,
    pub text: String,
}

impl ModeratorCredentials {
    pub fn valid(&self) -> bool {
        !self.login.is_empty() && !self.password.is_empty()
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ModeratorResponse {
    pub id: String,
    pub login: String,
}

impl From<AdminModel> for ModeratorResponse {
    fn from(value: AdminModel) -> Self {
        Self {
            id: value.id.to_string(),
            login: value.login,
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct ChatResponse {
    id: String,
}

impl From<ChatModel> for ChatResponse {
    fn from(value: ChatModel) -> Self {
        Self {
            id: value.id.to_string(),
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct GetChatRequest {
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct SendMessageResponse {
    pub message: Message,
    pub images_ids: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub text: String,
    pub sender: String,
    pub created_at: DateTime,
}

impl From<MessageModel> for Message {
    fn from(value: MessageModel) -> Self {
        Self {
            id: value.id.to_string(),
            chat_id: value.chat_id.to_string(),
            text: value.text,
            sender: serde_json::to_string(&value.sender).unwrap(),
            created_at: value.created_at,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ChatHistory {
    messages: Vec<(Message, Vec<String>)>,
}
impl From<Vec<(MessageModel, Vec<ImageModel>)>> for ChatHistory {
    fn from(value: Vec<(MessageModel, Vec<ImageModel>)>) -> Self {
        Self {
            messages: value
                .into_iter()
                .map(|(message, images)| {
                    {
                        (
                            Into::<Message>::into(message),
                            images
                                .into_iter()
                                .map(|image| image.id.to_string())
                                .collect::<Vec<_>>(),
                        )
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/admin/moderator",
    request_body = ModeratorCredentials,
    responses(
        (status = 200, description = "Moderator was successfully created", body = ModeratorResponse),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 409, description = "Login was occupied",                 body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn create_moderator(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Json(credentials): Json<ModeratorCredentials>,
) -> Response {
    if !credentials.valid() {
        return AppError::EmptyCredentials.into_response();
    }
    let parameters = CreateModeratorParameters {
        login: credentials.login,
        password: credentials.password,
    };
    match app_state.database_connection().begin().await {
        Ok(transaction) => match AdminService::create_moderator(parameters, &transaction).await {
            Ok(moderator) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                Json(Into::<ModeratorResponse>::into(moderator)).into_response()
            }
            Err(cause) => Into::<AppError>::into(cause).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/admin/moderator/{id}",
    responses(
        (status = 204, description = "Moderator was successfully deleted"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 403, description = "Moderator has admin role",           body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    params(("id" = i64, Path, description = "Moderator id")),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state, _admin))]
pub async fn delete_moderator(
    State(app_state): State<Arc<AppState>>,
    AdminAuthJWT(_admin): AdminAuthJWT,
    Path(moderator_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(transaction) => match AdminService::delete_moderator(moderator_id, &transaction).await {
            Ok(()) => {
                if let Err(cause) = transaction.commit().await {
                    return AppError::InternalServerError(Box::new(cause)).into_response();
                }
                StatusCode::NO_CONTENT.into_response()
            }
            Err(cause) => Into::<AppError>::into(cause).into_response(),
        },
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct AssignModeratorRequest {
    order_id: String,
}

#[utoipa::path(
    patch,
    path = "/api/admin/moderator/assign",
    request_body = AssignModeratorRequest,
    responses(
        (status = 204, description = "Moderator was successfully assigned to the order"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 403, description = "Moderator has admin role",           body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state, moderator))]
pub async fn assign_moderator(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Json(payload): Json<AssignModeratorRequest>,
) -> Response {
    let order_id = match payload.order_id.parse::<i64>() {
        Ok(order_id) => order_id,
        Err(cause) => {
            return Into::<AppError>::into(cause).into_response();
        }
    };

    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = AssignModeratorParameters {
                moderator_id: moderator.id,
                order_id,
            };
            match AdminService::assign_moderator(parameters, &transaction).await {
                Ok(()) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(cause) => Into::<AppError>::into(cause).into_response(),
            }
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema)]
pub struct UnassignModeratorRequest {
    order_id: String,
}

#[utoipa::path(
    patch,
    path = "/api/admin/moderator/unassign",
    request_body = UnassignModeratorRequest,
    responses(
        (status = 204, description = "Moderator was successfully unassigned from the order"),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 409, description = "Some conflict occurred",              body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
#[tracing::instrument(skip(app_state, moderator))]
pub async fn unassign_moderator(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Json(payload): Json<UnassignModeratorRequest>,
) -> Response {
    let order_id = match payload.order_id.parse::<i64>() {
        Ok(order_id) => order_id,
        Err(cause) => {
            return Into::<AppError>::into(cause).into_response();
        }
    };
    match app_state.database_connection().begin().await {
        Ok(transaction) => {
            let parameters = UnassignModeratorParameters {
                moderator_id: moderator.id,
                order_id,
            };
            match AdminService::unassign_moderator(parameters, &transaction).await {
                Ok(()) => {
                    if let Err(cause) = transaction.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }
                    StatusCode::NO_CONTENT.into_response()
                }
                Err(cause) => Into::<AppError>::into(cause).into_response(),
            }
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/moderator",
    responses(
        (status = 200, description = "Moderators were successfully retrieved", body = [ModeratorResponse]),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    )
)]
pub async fn list_moderators(State(app_state): State<Arc<AppState>>) -> Response {
    match AdminService::moderators(app_state.database_connection()).await {
        Ok(moderators) => Json(
            moderators
                .into_iter()
                .map(Into::<ModeratorResponse>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/moderator/orders",
    responses(
        (status = 200, description = "Orders were successfully retrieved", body = [Order]),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn list_moderators_orders(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
) -> Response {
    match AdminService::moderators_orders(moderator.id, app_state.database_connection()).await {
        Ok(moderators) => Json(
            moderators
                .into_iter()
                .map(Into::<Order>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/moderator/unassigned-orders",
    responses(
        (status = 200, description = "Orders were successfully retrieved", body = [Order]),
        (status = 400, description = "Bad request",                        body = Details),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 404, description = "Moderator was not found",            body = Details),
        (status = 500, description = "Internal Server Error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn list_unassigned_orders(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(_moderator): ModeratorAuthJWT,
) -> Response {
    match AdminService::unassigned_orders(app_state.database_connection()).await {
        Ok(orders) => Json(
            orders
                .into_iter()
                .map(Into::<Order>::into)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ModeratorOrAdminInfo {
    id: String,
    login: String,
    role: String,
}

impl From<AdminModel> for ModeratorOrAdminInfo {
    fn from(value: AdminModel) -> Self {
        Self {
            id: value.id.to_string(),
            login: value.login,
            role: serde_json::to_string(&value.role).unwrap(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/self",
    responses(
        (status = 200, description = "Information was successfully retrieved", body = ModeratorOrAdminInfo),
        (status = 401, description = "Unauthorized",                       body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn self_info(ModeratorAuthJWT(moderator): ModeratorAuthJWT) -> impl IntoResponse {
    Json(Into::<ModeratorOrAdminInfo>::into(moderator))
}

#[utoipa::path(
    patch,
    path = "/api/admin/moderator/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password was successfully changed"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn change_password(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Json(payload): Json<ChangePasswordRequest>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(connection) => {
            let parameters = ResetPasswordParameters {
                moderator_id: moderator.id,
                old_password: &payload.old_password,
                new_password: &payload.new_password,
            };
            if let Err(cause) = AuthService::reset_password(parameters, &connection).await {
                return Into::<AppError>::into(cause).into_response();
            }
            if let Err(cause) = connection.commit().await {
                return AppError::InternalServerError(Box::new(cause)).into_response();
            }
            (StatusCode::OK).into_response()
        }
        Err(cause) => AppError::InternalServerError(Box::new(cause)).into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/admin/moderator/chat",
    request_body = GetChatRequest,
    responses(
        (status = 200, description = "Chat was successfully retrieved",    body = ChatResponse),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn chat(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Json(payload): Json<GetChatRequest>,
) -> Response {
    let steam_id: i64 = match payload.id.parse() {
        Ok(id) => id,
        Err(cause) => return Into::<AppError>::into(cause).into_response(),
    };
    let params = GetChatParameters {
        moderator_id: moderator.id,
        steam_id,
    };

    match ChatService::chat(params, app_state.database_connection()).await {
        Ok(chat) => Json(Into::<ChatResponse>::into(chat)).into_response(),
        Err(cause) => Into::<AppError>::into(cause).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/admin/moderator/chat/{id}/message",
    request_body = UploadData,
    params(("id" = i64, Path, description = "Chat id")),

    responses(
        (status = 200, description = "Message was successfully sent",    body = SendMessageResponse),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
        (status = 403, description = "Moderator is not a member of this chat"),
        (status = 404, description = "Chat  was not found"),

    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn send_message(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Path(chat_id): Path<i64>,
    TypedMultipart(UploadData { image, text }): TypedMultipart<UploadData>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(connection) => {
            let _ = match ChatEntity::find_by_id(chat_id).one(&connection).await {
                Ok(Some(chat)) => match chat.moderator_id == moderator.id {
                    true => chat,
                    false => return StatusCode::FORBIDDEN.into_response(),
                },
                Ok(None) => return StatusCode::NOT_FOUND.into_response(),
                Err(_cause) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            let params = SendMessageParameters {
                folder: app_state.configuration().upload_folder().clone(),
                chat_id,
                sender: Sender::Moderator,
                text,
                image: image.as_ref(),
            };

            match ChatService::send_message(params, &connection).await {
                Ok(res) => {
                    if let Err(cause) = connection.commit().await {
                        return AppError::InternalServerError(Box::new(cause)).into_response();
                    }

                    let send = SendMessageResponse {
                        message: Into::<Message>::into(res.0),
                        images_ids: res.1.iter().map(|id| id.to_string()).collect(),
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
    path = "/api/admin/moderator/chat/{id}/history",
    params(("id" = i64, Path, description = "Chat id")),

    responses(
        (status = 200, description = "History was successfully retrieved", body = ChatHistory),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
        (status = 403, description = "Moderator is not a member of this chat"),
        (status = 404, description = "Chat was not found"),

    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn history(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Path(chat_id): Path<i64>,
) -> Response {
    match app_state.database_connection().begin().await {
        Ok(connection) => {
            let chat = match ChatEntity::find_by_id(chat_id).one(&connection).await {
                Ok(Some(chat)) => match chat.moderator_id == moderator.id {
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

use axum::extract::{
    ws::{Message as WsMessage, WebSocket},
    WebSocketUpgrade,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthQuery {
    pub authorization: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
    Query(AuthQuery { authorization }): Query<AuthQuery>,
) -> Response {
    let token = match authorization.split_once(' ') {
        Some(("Bearer", contents)) => contents.to_string(),
        _ => return AppError::AuthorizationHeaderBadSchema.into_response(),
    };

    let params = JwtCheckParams {
        token,
        secret: state.configuration().jwt_secret(),
    };

    let claims = match AuthService::check(params) {
        Ok(claims) => claims,
        Err(cause) => return Into::<AppError>::into(cause).into_response(),
    };

    let moderator = match AdminEntity::find_by_id(claims.sub)
        .one(state.database_connection())
        .await
    {
        Ok(Some(admin)) => admin,
        Ok(None) => return AppError::Unauthorized.into_response(),
        Err(cause) => return AppError::InternalServerError(Box::new(cause)).into_response(),
    };

    match ChatEntity::find_by_id(chat_id)
        .one(state.database_connection())
        .await
    {
        Ok(Some(chat)) => {
            if chat.moderator_id != moderator.id {
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

#[utoipa::path(
    get,
    path = "/api/user/chat/{id}/image/{id}",
    params(("id" = (i64, i64), Path, description = "Chat id and image id")),

    responses(
        (status = 200, description = "Image was successfully retrieved"),
        (status = 401, description = "Unauthorized",                       body = Details),
        (status = 500, description = "Internal server error",              body = Details),
        (status = 403, description = "Moderator is not a member of this chat"),
        (status = 404, description = "Chat was not found"),

    ),
    security(
        ("jwt_admin" = [])
    )
)]
pub async fn image(
    State(app_state): State<Arc<AppState>>,
    ModeratorAuthJWT(moderator): ModeratorAuthJWT,
    Path((chat_id, image_id)): Path<(i64, i64)>,
) -> Response {
    let _chat = match ChatEntity::find_by_id(chat_id)
        .one(app_state.database_connection())
        .await
    {
        Ok(Some(chat)) => match chat.moderator_id == moderator.id {
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
