use entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};

use migration::{Alias, Query, SimpleExpr};
use sea_orm::{
    prelude::*, Condition, FromQueryResult, JoinType, Order, QueryOrder, QuerySelect, Set,
    TransactionTrait,
};

use crate::errors::AppError;

#[allow(dead_code)]
pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error("User with id = {0} was not found")]
    UserWasNotFound(i64),
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::UserWasNotFound(id) => AppError::UserWasNotFound(id),
        }
    }
}

#[derive(FromQueryResult)]
pub struct TopUser {
    pub steam_id: i64,
    pub amount: Decimal,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn safe_registration<T>(
        steam_id: i64,
        connection: &T,
    ) -> Result<UserModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match UserEntity::find_by_id(steam_id).one(connection).await? {
            Some(user) => Ok(user),
            None => {
                let user_to_be_inserted = UserActiveModel {
                    steam_id: Set(steam_id),
                    ..Default::default()
                };
                Ok(UserEntity::insert(user_to_be_inserted)
                    .exec_with_returning(connection)
                    .await?)
            }
        }
    }

    pub async fn get_by_steam_id<T>(
        steam_id: i64,
        connection: &T,
    ) -> Result<Option<UserModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(UserEntity::find_by_id(steam_id).one(connection).await?)
    }

    pub async fn set_email<T>(
        steam_id: i64,
        email: Option<String>,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let user = match UserEntity::find_by_id(steam_id).one(connection).await? {
            Some(user) => Ok(user),
            None => Err(ServiceError::UserWasNotFound(steam_id)),
        }?;

        let mut user_to_be_changed: UserActiveModel = user.into();
        user_to_be_changed.email = Set(email);

        user_to_be_changed.update(connection).await?;
        Ok(())
    }

    pub async fn set_trade_url<T>(
        steam_id: i64,
        trade_url: Option<String>,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let user = match UserEntity::find_by_id(steam_id).one(connection).await? {
            Some(user) => Ok(user),
            None => Err(ServiceError::UserWasNotFound(steam_id)),
        }?;

        let mut user_to_be_changed: UserActiveModel = user.into();
        user_to_be_changed.trade_url = Set(trade_url);

        user_to_be_changed.update(connection).await?;
        Ok(())
    }

    pub async fn top<T>(
        limit: u64,
        offset: u64,
        connection: &T,
    ) -> Result<Vec<TopUser>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(UserEntity::find()
            .select_only()
            .column(entity::user::Column::SteamId)
            .column_as(entity::order::Column::Amount.sum(), "amount")
            .filter(
                Condition::any().add(
                    UserColumn::SteamId.not_in_subquery(
                        Query::select()
                            .column(entity::blacklisted::Column::SteamId)
                            .from(entity::blacklisted::Entity)
                            .to_owned(),
                    ),
                ),
            )
            .join_as(
                JoinType::LeftJoin,
                entity::user::Relation::Order.def(),
                Alias::new("order"),
            )
            .group_by(UserColumn::SteamId)
            .order_by(SimpleExpr::Custom("amount".to_owned()), Order::Desc)
            .offset(Some(offset))
            .limit(Some(limit))
            .into_model::<TopUser>()
            .all(connection)
            .await?)
    }

    pub async fn registered_in_period<T>(
        period: (chrono::NaiveDateTime, chrono::NaiveDateTime),
        connection: &T,
    ) -> Result<u64, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(UserEntity::find()
            .filter(UserColumn::RegisteredAt.between(period.0, period.1))
            .count(connection)
            .await?)
    }

    pub async fn avatar<T>(steam_id: i64, connection: &T) -> Result<Option<String>, ServiceError>
    where
        T: TransactionTrait + ConnectionTrait,
    {
        tracing::info!("!!!");
        match UserEntity::find_by_id(steam_id).one(connection).await? {
            Some(user) => Ok(user.avatar_url),
            None => Err(ServiceError::UserWasNotFound(steam_id)),
        }
    }

    pub async fn username<T>(steam_id: i64, connection: &T) -> Result<Option<String>, ServiceError>
    where
        T: TransactionTrait + ConnectionTrait,
    {
        match UserEntity::find_by_id(steam_id).one(connection).await? {
            Some(user) => Ok(user.username),
            None => Err(ServiceError::UserWasNotFound(steam_id)),
        }
    }
}
