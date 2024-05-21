use crate::{
    commands::Command,
    handlers::{
        handle_user_login, handle_user_password, logout, process_all_callback_queries, start,
    },
    state::State,
};
use teloxide::{
    dispatching::{
        dialogue::{self, serializer::Json, SqliteStorage},
        UpdateFilterExt, UpdateHandler,
    },
    dptree::{self},
    types::Update,
};

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Logout])
        .endpoint(logout);

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveLogin { message_id }].endpoint(handle_user_login))
        .branch(case![State::ReceivePassword { login, message_id }].endpoint(handle_user_password));

    let callback_query_handler =
        Update::filter_callback_query().endpoint(process_all_callback_queries);

    dialogue::enter::<Update, SqliteStorage<Json>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
