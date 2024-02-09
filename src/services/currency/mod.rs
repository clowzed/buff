use crate::errors::AppError;
use entity::currency_rate::{
    ActiveModel as CurrencyRateActiveModel, Column as CurrencyRateColumn,
    Entity as CurrencyRateEntity, Model as CurrencyRateModel,
};
use sea_orm::{prelude::*, Set, TransactionTrait};
use std::fmt::Debug;

#[allow(dead_code)]
pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Currency symbol was not found")]
    SymbolNotFound,
    #[error("Currency symbol already exists")]
    SymbolAlreadyExists,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::SymbolNotFound => AppError::SymbolNotFound,
            ServiceError::SymbolAlreadyExists => AppError::SymbolAlreadyExists,
        }
    }
}

#[derive(Debug)]
pub struct CreateCurrencyRateParameters {
    pub symbol: String,
    pub rate: Decimal,
}

#[derive(Debug)]
pub struct SetCurrencyRateParameters {
    pub id: i64,
    pub rate: Decimal,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn currency_rate<T>(
        symbol: &str,
        connection: &T,
    ) -> Result<CurrencyRateModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match CurrencyRateEntity::find()
            .filter(CurrencyRateColumn::Symbol.eq(symbol))
            .one(connection)
            .await?
        {
            Some(rate) => Ok(rate),
            None => Err(ServiceError::SymbolNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn create<T>(
        parameters: CreateCurrencyRateParameters,
        connection: &T,
    ) -> Result<CurrencyRateModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match CurrencyRateEntity::find()
            .filter(CurrencyRateColumn::Symbol.eq(&parameters.symbol))
            .one(connection)
            .await?
        {
            Some(_rate) => Err(ServiceError::SymbolAlreadyExists),
            None => {
                let currency_rate_to_be_inserted = CurrencyRateActiveModel {
                    symbol: Set(parameters.symbol),
                    rate: Set(parameters.rate),
                    ..Default::default()
                };
                Ok(CurrencyRateEntity::insert(currency_rate_to_be_inserted)
                    .exec_with_returning(connection)
                    .await?)
            }
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn set_rate<T>(
        parameters: SetCurrencyRateParameters,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match CurrencyRateEntity::find_by_id(parameters.id)
            .one(connection)
            .await?
        {
            Some(rate) => {
                let mut currency_rate_to_be_updated: CurrencyRateActiveModel = rate.into();
                currency_rate_to_be_updated.rate = Set(parameters.rate);
                currency_rate_to_be_updated.update(connection).await?;
                Ok(())
            }
            None => Err(ServiceError::SymbolNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn all<T>(connection: &T) -> Result<Vec<CurrencyRateModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(CurrencyRateEntity::find().all(connection).await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn delete<T>(id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let rate_to_be_deleted = match CurrencyRateEntity::find_by_id(id).one(connection).await? {
            Some(rate) => Ok(rate),
            None => Err(ServiceError::SymbolNotFound),
        }?;

        rate_to_be_deleted.delete(connection).await?;
        Ok(())
    }

    #[tracing::instrument(skip(connection))]
    pub async fn get<T>(id: i64, connection: &T) -> Result<CurrencyRateModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match CurrencyRateEntity::find_by_id(id).one(connection).await? {
            Some(rate) => Ok(rate),
            None => Err(ServiceError::SymbolNotFound),
        }
    }
}
