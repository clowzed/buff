use serde::{Deserialize, Serialize};
use teloxide::types::MessageId;

#[derive(Serialize, Deserialize, Clone)]
pub enum State {
    Start {
        message_id: Option<MessageId>,
    },
    ReceiveLogin {
        message_id: MessageId,
    },
    ReceivePassword {
        message_id: MessageId,
        login: String,
    },
}

impl Default for State {
    fn default() -> Self {
        State::Start { message_id: None }
    }
}

impl State {
    pub fn message_id(&self) -> Option<MessageId> {
        match self {
            State::Start { message_id } => message_id.clone(),
            State::ReceiveLogin { message_id } => Some(message_id.clone()),
            State::ReceivePassword { message_id, .. } => Some(message_id.clone()),
        }
    }
}
