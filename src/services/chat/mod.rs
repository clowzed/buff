use axum::body::Bytes;
use axum_typed_multipart::FieldData;
use entity::{
    chat::{
        ActiveModel as ChatActiveModel, Column as ChatColumn, Entity as ChatEntity,
        Model as ChatModel,
    },
    image::{
        ActiveModel as ImageActiveModel, Column as ImageColumn, Entity as ImageEntity,
        Model as ImageModel,
    },
    message::{
        ActiveModel as MessageActiveModel, Column as MessageColumn, Entity as MessageEntity,
        Model as MessageModel,
    },
    sea_orm_active_enums::Sender as MessageSender,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use std::{fmt::Debug, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct GetChatParameters {
    pub moderator_id: i64,
    pub steam_id: i64,
    pub order_id: i64,
}

pub enum Sender {
    Moderator,
    User,
}

pub struct UploadImagesData<'a> {
    pub folder: PathBuf,
    pub image: Option<&'a FieldData<Bytes>>,
    pub message_id: i64,
}

pub struct SendMessageParameters<'a> {
    pub folder: PathBuf,
    pub chat_id: i64,
    pub sender: Sender,
    pub text: String,
    pub image: Option<&'a FieldData<Bytes>>,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn chat<T>(
        parameters: impl Into<GetChatParameters> + Debug,
        connection: &T,
    ) -> Result<ChatModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let params = parameters.into();
        match ChatEntity::find()
            .filter(
                ChatColumn::ModeratorId
                    .eq(params.moderator_id)
                    .and(ChatColumn::SteamId.eq(params.steam_id))
                    .and(ChatColumn::OrderId.eq(params.order_id)),
            )
            .one(connection)
            .await?
        {
            Some(chat) => Ok(chat),
            None => {
                let new_chat = ChatActiveModel {
                    steam_id: Set(params.steam_id),
                    moderator_id: Set(params.moderator_id),
                    order_id: Set(params.order_id),
                    ..Default::default()
                };

                Ok(ChatEntity::insert(new_chat)
                    .exec_with_returning(connection)
                    .await?)
            }
        }
    }

    #[tracing::instrument(skip(connection, parameters))]
    pub async fn upload_images<'a, T>(
        parameters: UploadImagesData<'a>,
        connection: &T,
    ) -> Result<Vec<i64>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        if parameters.image.is_none() {
            return Ok(Vec::new());
        }

        let path = {
            let real_filepath = parameters.folder.join(format!(
                "{}-{}",
                uuid::Uuid::new_v4(),
                uuid::Uuid::new_v4()
            ));
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&real_filepath)
                .await?;
            file.write(&parameters.image.unwrap().contents).await?;

            real_filepath.display().to_string()
        };

        let active_model = ImageActiveModel {
            message_id: Set(parameters.message_id),
            path: Set(path.clone()),
            ..Default::default()
        };

        ImageEntity::insert(active_model).exec(connection).await?;
        Ok(ImageEntity::find()
            .filter(ImageColumn::MessageId.eq(parameters.message_id))
            .all(connection)
            .await?
            .into_iter()
            .map(|image| image.id)
            .collect())
    }

    #[tracing::instrument(skip(connection, parameters))]
    pub async fn send_message<T>(
        parameters: impl Into<SendMessageParameters<'_>>,
        connection: &T,
    ) -> Result<(MessageModel, Vec<i64>), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let params = parameters.into();

        let message_to_be_inserted = MessageActiveModel {
            chat_id: Set(params.chat_id),
            text: Set(params.text),
            sender: match params.sender {
                Sender::Moderator => Set(MessageSender::Moderator),
                Sender::User => Set(MessageSender::User),
            },
            ..Default::default()
        };

        let message = MessageEntity::insert(message_to_be_inserted)
            .exec_with_returning(connection)
            .await?;

        let parameters = UploadImagesData {
            folder: params.folder,
            image: params.image,
            message_id: message.id,
        };

        match Self::upload_images(parameters, connection).await {
            Ok(ids) => Ok((message, ids)),
            Err(cause) => {
                Into::<MessageActiveModel>::into(message)
                    .delete(connection)
                    .await
                    .ok(); // Try to delete and if failed will just not show images
                Err(cause)
            }
        }
    }

    pub async fn history<T>(
        chat_id: i64,
        connection: &T,
    ) -> Result<Vec<(MessageModel, Vec<ImageModel>)>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(MessageEntity::find()
            .filter(MessageColumn::ChatId.eq(chat_id))
            .find_with_related(ImageEntity)
            .all(connection)
            .await?)
    }
}
