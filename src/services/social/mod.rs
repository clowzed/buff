use crate::errors::AppError;
use entity::social::ActiveModel as SocialActiveModel;

use entity::social::Entity as SocialEntity;
use entity::social::Model as SocialModel;
use sea_orm::prelude::*;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use std::fmt::Debug;

#[allow(dead_code)]
pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Name was not found")]
    NameWasNotFound,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::NameWasNotFound => AppError::NameWasNotFound,
        }
    }
}

#[derive(Debug)]
pub struct SetSocialParameters {
    pub id: i64,
    pub url: Option<String>,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn all<T>(connection: &T) -> Result<Vec<SocialModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(SocialEntity::find().all(connection).await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn set<T>(
        parameters: impl Into<SetSocialParameters> + Debug,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let parameters = parameters.into();

        let social = match SocialEntity::find_by_id(parameters.id)
            .one(connection)
            .await?
        {
            Some(social) => Ok(social),
            None => Err(ServiceError::NameWasNotFound),
        }?;

        let mut social_to_be_updated: SocialActiveModel = social.into();

        social_to_be_updated.url = Set(parameters.url);
        social_to_be_updated.update(connection).await?;
        Ok(())
    }
}
