use crate::errors::AppError;
use entity::requisites::{
    ActiveModel as RequisitesActiveModel, Column as RequisitesColumn, Entity as RequisitesEntity,
    Model as RequisitesModel,
};
use sea_orm::{prelude::*, Order::Asc, QueryOrder, Set, TransactionTrait};
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
pub struct SetRequisitesParameters {
    pub id: i64,
    pub data: Option<String>,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn all<T>(connection: &T) -> Result<Vec<RequisitesModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(RequisitesEntity::find()
            .order_by(RequisitesColumn::Id, Asc)
            .all(connection)
            .await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn set<T>(
        parameters: impl Into<SetRequisitesParameters> + Debug,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let parameters = parameters.into();

        let requisites = match RequisitesEntity::find_by_id(parameters.id)
            .one(connection)
            .await?
        {
            Some(requisites) => Ok(requisites),
            None => Err(ServiceError::NameWasNotFound),
        }?;

        let mut requisites_to_be_updated: RequisitesActiveModel = requisites.into();

        requisites_to_be_updated.data = Set(parameters.data);
        requisites_to_be_updated.update(connection).await?;
        Ok(())
    }
}
