use crate::apiservice::ApiService;
use crate::Config;
use crate::{repository::Repository, state::State, HandlerResult, ModeratorId, MyDialogue};
use buffapi::apis::configuration::Configuration;
use buffapi::models::ModeratorOrAdminInfo;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;
use teloxide::types::MessageId;
use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::{EditMessageTextSetters, SendMessageSetters},
    requests::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};
use tokio::sync::RwLock;

//? Renaming is used here to prevent error
//? because inline data can be only 64 bytes
#[derive(Serialize, Deserialize)]
pub enum CallbackData {
    Subscribe,
    #[serde(rename = "a")]
    AcceptModerator {
        #[serde(rename = "a")]
        moderator_id: ModeratorId,
        #[serde(rename = "b")]
        user_id: ChatId,
        #[serde(rename = "c")]
        message_id: MessageId,
    },
    #[serde(rename = "d")]
    DeclineModerator {
        #[serde(rename = "e")]
        moderator_id: ModeratorId,
        #[serde(rename = "f")]
        user_id: ChatId,
        #[serde(rename = "g")]
        message_id: MessageId,
    },
}

impl Display for CallbackData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            CallbackData::Subscribe => "🔔 Подписаться на уведомления",
            CallbackData::AcceptModerator { .. } => "✅ Подтвердить",
            CallbackData::DeclineModerator { .. } => "❌ Отклонить",
        };
        write!(f, "{}", data)
    }
}

pub async fn start(bot: Bot, dialogue: MyDialogue, message: Message) -> HandlerResult {
    if let Ok(Some(state)) = dialogue.get().await {
        if let Some(message_id) = state.message_id() {
            bot.delete_message(message.chat.id, message_id).await.ok();
        }
    }

    bot.delete_message(message.chat.id, message.id).await.ok();

    let options = [CallbackData::Subscribe].iter().map(|option| {
        InlineKeyboardButton::callback(
            option.to_string(),
            serde_json::to_string(&option).expect("Failed to convert CallbackData to json!"),
        )
    });

    bot.send_message(
        message.chat.id,
        "Здравствуйте!\n🔔 Нажмите, чтобы войти в качестве модератора и начать получать уведомления!",
    )
    .reply_markup(InlineKeyboardMarkup::new([options]))
    .await
    .ok();
    Ok(())
}

pub async fn logout(
    bot: Bot,
    dialogue: MyDialogue,
    repository: Arc<RwLock<Repository<ModeratorId, ChatId>>>,
    message: Message,
) -> HandlerResult {
    if let Ok(Some(state)) = dialogue.get().await {
        if let Some(message_id) = state.message_id() {
            bot.delete_message(message.chat.id, message_id).await.ok();
        }
    }

    bot.delete_message(message.chat.id, message.id).await.ok();

    let message_about_logout = {
        let read_guard = repository.read().await;

        if let Some((ModeratorId(moderator_id), _chat_id)) =
            read_guard.get_by_value(message.chat.id).await
        {
            //? This prevents deadlock
            drop(read_guard);

            repository
                .write()
                .await
                .remove(ModeratorId(moderator_id))
                .await;
            format!("🟢 Вы отписались от уведомлений для модератора {moderator_id}!\nНажмите кнопку, чтобы подписаться снова!")
        } else {
            format!("🔴 Вы не были подписаны на уведомления!\nНажмите кнопку, чтобы подписаться!")
        }
    };

    let options = [CallbackData::Subscribe].iter().map(|option| {
        InlineKeyboardButton::callback(
            option.to_string(),
            serde_json::to_string(&option).expect("Failed to convert CallbackData to json!"),
        )
    });

    bot.send_message(message.chat.id, message_about_logout)
        .reply_markup(InlineKeyboardMarkup::new([options]))
        .await
        .ok();
    Ok(())
}

pub async fn process_subscribe_for_notifications_inline(
    bot: Bot,
    dialogue: MyDialogue,
    repository: Arc<RwLock<Repository<ModeratorId, ChatId>>>,
    q: CallbackQuery,
) -> HandlerResult {
    //* That is brilliant to avoid let chains :)
    if let (Some(chat_id), Some(message)) = (q.chat_id(), q.message) {
        //* Answer callback for removing timer icon
        bot.answer_callback_query(q.id).await?;

        //* Check if user has already been subscribed for notifications
        match repository.read().await.get_by_value(chat_id).await {
            //* If he is subscribed just remind him
            Some(_moderator_id) => {
                let already_subscribed_text = "🔔 Вы уже получаете уведомления! Если вы хотите отписаться, отправьте команду /logout";
                bot.edit_message_text(chat_id, message.id, already_subscribed_text)
                    .reply_markup(InlineKeyboardMarkup::default())
                    .await
                    .ok();
            }
            None => {
                //* Prompt for a login
                let login_prompt = "🔒 Пожалуйста, отправьте логин!";
                bot.edit_message_text(chat_id, message.id, login_prompt)
                    .reply_markup(InlineKeyboardMarkup::default())
                    .await
                    .ok();

                //* Change state to process login
                dialogue
                    .update(State::ReceiveLogin {
                        message_id: message.id,
                    })
                    .await
                    .ok();
            }
        }
    }
    Ok(())
}

pub async fn process_all_callback_queries(
    bot: Bot,
    dialogue: MyDialogue,
    repository: Arc<RwLock<Repository<ModeratorId, ChatId>>>,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(ref data) = q.data {
        if let Ok(cdata) = serde_json::from_str::<CallbackData>(&data) {
            match cdata {
                CallbackData::Subscribe => {
                    process_subscribe_for_notifications_inline(bot, dialogue, repository, q).await?
                }
                CallbackData::AcceptModerator { .. } => {
                    process_accept_moderator_for_notifications_inline(bot, repository, q, cdata)
                        .await?
                }
                CallbackData::DeclineModerator { .. } => {
                    process_decline_moderator_for_notifications_inline(bot, q, cdata).await?
                }
            }
        }
    }
    Ok(())
}

pub async fn process_accept_moderator_for_notifications_inline(
    bot: Bot,
    repository: Arc<RwLock<Repository<ModeratorId, ChatId>>>,
    q: CallbackQuery,
    data: CallbackData,
) -> HandlerResult {
    //* That is brilliant to avoid let chains :)
    if let (
        Some(chat_id),
        Some(message),
        CallbackData::AcceptModerator {
            moderator_id,
            user_id,
            message_id: bot_message_id_in_user_chat,
        },
    ) = (q.chat_id(), q.message, data)
    {
        //* Answer callback for removing timer icon
        bot.answer_callback_query(q.id).await?;

        //* Insert user for notifications
        repository
            .write()
            .await
            .insert(moderator_id.clone(), user_id)
            .await;

        //* Edit message from admin

        let moderator_id_raw = moderator_id.0;
        let accepted_notification_for_admin = format!("🟢 Модератор успешно подтвержден!\nПользователь {user_id} теперь получает уведомления для модератора {moderator_id_raw}");
        bot.edit_message_text(chat_id, message.id, accepted_notification_for_admin)
            .reply_markup(InlineKeyboardMarkup::default())
            .await?;

        //* Notify user that he was accepted
        let accepted_notification_for_user = format!("🔓 Корректность данных подтверждена.\n🟢 Вход разрешен.\n🟢 Администратор принял вашу заявку!\nВы будете получать уведомления для модератора {moderator_id_raw}.\nДля того, чтобы отписаться от уведомлений отправьте команду /logout");
        bot.edit_message_text(
            user_id,
            bot_message_id_in_user_chat,
            accepted_notification_for_user,
        )
        .reply_markup(InlineKeyboardMarkup::default())
        .await?;
    }
    Ok(())
}

pub async fn process_decline_moderator_for_notifications_inline(
    bot: Bot,
    q: CallbackQuery,
    data: CallbackData,
) -> HandlerResult {
    //* That is brilliant to avoid let chains :)
    if let (
        Some(chat_id),
        Some(message),
        CallbackData::DeclineModerator {
            moderator_id: _,
            user_id,
            message_id: bot_message_id_in_user_chat,
        },
    ) = (q.chat_id(), q.message, data)
    {
        //* Answer callback for removing timer icon
        bot.answer_callback_query(q.id).await?;

        //* Edit message from admin

        let accepted_notification_for_admin = format!("🔴 Модератор был отклонен!");
        bot.edit_message_text(chat_id, message.id, accepted_notification_for_admin)
            .reply_markup(InlineKeyboardMarkup::default())
            .await?;

        //* Notify user that he was accepted
        let accepted_notification_for_user = format!("🔓 Корректность данных подтверждена.\n🔴 Вход запрещен.\n🔴 Администратор не принял вашу заявку!");
        bot.edit_message_text(
            user_id,
            bot_message_id_in_user_chat,
            accepted_notification_for_user,
        )
        .reply_markup(InlineKeyboardMarkup::default())
        .await?;
    }
    Ok(())
}

pub async fn handle_user_login(bot: Bot, dialogue: MyDialogue, message: Message) -> HandlerResult {
    let message_text = message.text();
    //* Safe as it is guarded by case in schema
    let state = dialogue.get().await?;

    if let (
        Some(State::ReceiveLogin {
            message_id: bot_message_id,
        }),
        Some(message_text),
    ) = (state, message_text)
    {
        //* Check if text was sent
        let chat_id = message.chat.id;
        let user_message_id = message.id;

        //* Delete message from user containing login
        bot.delete_message(chat_id, user_message_id).await.ok();

        let password_prompt = "🔑 Пожалуйста, отправьте пароль!";

        //* Ask user for password
        bot.edit_message_text(chat_id, bot_message_id, password_prompt)
            .reply_markup(InlineKeyboardMarkup::default())
            .await
            .ok();

        //* Update state to wait for password
        dialogue
            .update(State::ReceivePassword {
                login: message_text.into(),
                message_id: bot_message_id,
            })
            .await?;
    }
    Ok(())
}

//? Yeah this one is a bit big
//? But well commented )
pub async fn handle_user_password(
    bot: Bot,
    dialogue: MyDialogue,
    message: Message,
    config: Arc<Config>,
) -> HandlerResult {
    let current_state = dialogue.get().await?;
    let message_text = message.text();

    if let (
        Some(State::ReceivePassword {
            message_id: bot_message_id,
            login,
        }),
        Some(password),
    ) = (current_state, message_text)
    {
        let chat_id = message.chat.id;
        let user_message_id = message.id;

        //* Delete message from user containing password
        bot.delete_message(chat_id, user_message_id).await.ok();

        //* Informing user that we need to check credentials
        let wait_check_placeholder = "🔐 Проверяем доступ...";

        bot.edit_message_text(chat_id, bot_message_id, wait_check_placeholder)
            .reply_markup(InlineKeyboardMarkup::default())
            .await
            .ok();

        //* Helper closure to avoid unnecessary code writing
        let access_denied_notification = || async {
            let bad_credentials_notification =
            "🔒 Корректность данных не подтверждена.\n🔴 Вход запрещен.\nПожалуйста, отправьте сначала корректный логин и затем пароль!";

            bot.edit_message_text(chat_id, bot_message_id, bad_credentials_notification)
                .reply_markup(InlineKeyboardMarkup::default())
                .await
                .ok();

            dialogue
                .update(State::ReceiveLogin {
                    message_id: bot_message_id,
                })
                .await
                .ok();
        };

        //* Setting up api client
        let mut configuration = Configuration::new();
        configuration.base_path = config.server_url.to_string();

        let mut api = ApiService::new(configuration);

        //* Trying to login with credentials
        let _token = match api.login(&login, password).await {
            Ok(token) => token,
            Err(cause) => {
                tracing::warn!(%cause, "Failed to login!");
                access_denied_notification().await;
                return Ok(());
            }
        };

        //* Trying to get moderator info
        let ModeratorOrAdminInfo {
            id: moderator_id,
            login: moderator_login,
            role: _,
        } = match api.moderator_info().await {
            Ok(info) => info,
            Err(cause) => {
                tracing::warn!(%cause, "Failed to get moderation info!");
                access_denied_notification().await;
                return Ok(());
            }
        };

        //* Notify user that he needs to wait for admin approval
        let wait_for_admin_approval_notification = "🔓 Корректность данных подтверждена.\n🟢 Вход разрешен.\n⏳ Дождитесь, когда администратор примет Вашу заявку.";

        bot.edit_message_text(
            chat_id,
            bot_message_id,
            wait_for_admin_approval_notification,
        )
        .reply_markup(InlineKeyboardMarkup::default())
        .await
        .ok();

        //* Notify admin about new request
        let new_request_notification = format!("➕ Запрос на добавление модератора\n👤 ID модератора: {moderator_id}\n👤 Логин модератора: {moderator_login}\n👤 ID пользователя: {chat_id}");

        let reply_markup = {
            let options = [
                CallbackData::AcceptModerator {
                    moderator_id: ModeratorId(moderator_id.parse().unwrap()), // Safe by api
                    user_id: chat_id,
                    message_id: bot_message_id,
                },
                CallbackData::DeclineModerator {
                    moderator_id: ModeratorId(moderator_id.parse().unwrap()), // Safe by api
                    user_id: chat_id,
                    message_id: bot_message_id,
                },
            ]
            .map(|option| {
                InlineKeyboardButton::callback(
                    option.to_string(),
                    serde_json::to_string(&option).unwrap(), //? Safe as I trust serde for simple enums :)
                )
            });

            InlineKeyboardMarkup::new([options])
        };
        bot.send_message(config.admin_id(), new_request_notification)
            .reply_markup(reply_markup)
            .await
            .ok();

        dialogue.exit().await?;
    }
    Ok(())
}
