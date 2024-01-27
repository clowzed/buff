use entity::blacklisted::ActiveModel as BlacklistedActiveModel;
use entity::blacklisted::Entity as BlacklistedEntity;

use entity::user::Entity as UserEntity;
use sea_orm::Set;
use sea_orm::{prelude::*, TransactionTrait};

use crate::errors::AppError;

#[allow(dead_code)]
pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error("User with id = {0} was not found")]
    UserWasNotFound(i64),
    #[error("User has already been blacklisted")]
    UserAlreadyBlacklisted,
    #[error("User is not blacklisted")]
    UserNotBlacklisted,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::UserWasNotFound(id) => AppError::UserWasNotFound(id),
            ServiceError::UserAlreadyBlacklisted => AppError::UserAlreadyBlacklisted,
            ServiceError::UserNotBlacklisted => AppError::UserNotBlacklisted,
        }
    }
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn blacklist_user<T>(steam_id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match UserEntity::find_by_id(steam_id).one(connection).await? {
            None => Err(ServiceError::UserWasNotFound(steam_id)),
            Some(user) => match user.find_related(BlacklistedEntity).one(connection).await? {
                Some(_) => Err(ServiceError::UserAlreadyBlacklisted),
                None => {
                    let user_to_be_blacklisted = BlacklistedActiveModel {
                        steam_id: Set(steam_id),
                        ..Default::default()
                    };

                    BlacklistedEntity::insert(user_to_be_blacklisted)
                        .exec(connection)
                        .await?;
                    Ok(())
                }
            },
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn unblacklist_user<T>(steam_id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match UserEntity::find_by_id(steam_id).one(connection).await? {
            None => Err(ServiceError::UserWasNotFound(steam_id)),
            Some(user) => match user.find_related(BlacklistedEntity).one(connection).await? {
                None => Err(ServiceError::UserNotBlacklisted),
                Some(blacklisted_user) => {
                    let user_to_be_unblacklisted: BlacklistedActiveModel = blacklisted_user.into();

                    BlacklistedEntity::delete(user_to_be_unblacklisted)
                        .exec(connection)
                        .await?;
                    Ok(())
                }
            },
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn all<T>(connection: &T) -> Result<Vec<i64>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        //? I thought that creating custom
        //? select with partial columns
        //? will be a bit overhead
        //? We do not have high load
        //? at this endpoint

        //* Sadly we cannot get rid of
        //* id as primary key because
        //* derive on entity fails to compile
        //* And we still need an ability
        //* to call user.find_related(BlacklistedEntity)
        Ok(BlacklistedEntity::find()
            .all(connection)
            .await?
            .into_iter()
            .map(|model| model.steam_id)
            .collect::<Vec<_>>())
    }
}
