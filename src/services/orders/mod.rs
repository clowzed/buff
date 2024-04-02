use crate::errors::AppError;

use chrono::Utc;
use entity::{
    admin::{Column as AdminColumn, Entity as AdminEntity, Relation as AdminRelation},
    order::{
        ActiveModel as OrderActiveModel, Column as OrderColumn, Entity as OrderEntity,
        Model as OrderModel,
    },
    requisites::Entity as RequisitesEntity,
    sea_orm_active_enums::{Role, Status},
};
use migration::Alias;
use sea_orm::RelationTrait;
use sea_orm::{
    prelude::Decimal, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait, TryIntoModel,
};
use std::fmt::Debug;

pub struct Service;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Order was not found")]
    OrderNotFound,
    #[error("Order has already been marked as succeeded")]
    OrderAlreadySucceeded,
    #[error("Order has already been marked as canceled")]
    OrderAlreadyCanceled,
    #[error("Requisites were not found by id")]
    RequisitesNotFound,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::OrderNotFound => AppError::OrderWasNotFound,
            ServiceError::OrderAlreadySucceeded => AppError::OrderAlreadySucceeded,
            ServiceError::OrderAlreadyCanceled => AppError::OrderAlreadyCanceled,
            ServiceError::RequisitesNotFound => AppError::RequisitesWereNotFound,
        }
    }
}

#[derive(Debug)]
pub struct CreateOrderParameters {
    pub steam_id: i64,
    pub payment_method: String,
    pub amount: Decimal,
    pub symbol: String,
    pub currency_rate: Decimal,
    pub requisites_id: i64,
}

#[derive(Debug)]
pub struct CancelOrderParameters {
    pub steam_id: i64,
    pub order_id: i64,
}

#[derive(Debug)]
pub struct MayBePayedOrderParameters {
    pub steam_id: i64,
    pub order_id: i64,
}
#[derive(Debug)]
pub struct GetUserOrderParameters {
    pub steam_id: i64,
    pub order_id: i64,
}

#[derive(Debug)]
pub struct SetOrderRequisitesParameters {
    pub steam_id: i64,
    pub order_id: i64,
    pub requisites: Option<String>,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn create_order<T>(
        parameters: impl Into<CreateOrderParameters> + Debug,
        connection: &T,
    ) -> Result<OrderModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let params: CreateOrderParameters = parameters.into();

        let moderator = AdminEntity::find()
            .filter(AdminColumn::Role.eq(Role::Moderator))
            .join_as(
                sea_orm::JoinType::LeftJoin,
                AdminRelation::Order.def(),
                Alias::new("a"),
            )
            .group_by(AdminColumn::Id)
            .order_by_asc(AdminColumn::Id.count())
            .limit(1)
            .one(connection)
            .await?
            .map(|moderator| moderator.id);

        let requisites = match RequisitesEntity::find_by_id(params.requisites_id)
            .one(connection)
            .await?
        {
            Some(r) => r,
            None => return Err(ServiceError::RequisitesNotFound),
        };

        let order_to_be_inserted = OrderActiveModel {
            steam_id: Set(params.steam_id),
            payment_method: Set(params.payment_method),
            status: Set(Status::Created),
            amount: Set(params.amount),
            currency_symbol: Set(params.symbol),
            fixed_currency_rate: Set(params.currency_rate),
            moderator_id: Set(moderator),
            requisites_id: Set(requisites.id),
            ..Default::default()
        };

        Ok(OrderEntity::insert(order_to_be_inserted)
            .exec_with_returning(connection)
            .await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn cancel_order<T>(
        parameters: impl Into<CancelOrderParameters> + Debug,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let params = parameters.into();

        match OrderEntity::find()
            .filter(
                OrderColumn::SteamId
                    .eq(params.steam_id)
                    .and(OrderColumn::Id.eq(params.order_id)),
            )
            .one(connection)
            .await?
        {
            Some(order) => match order.status {
                Status::Succeeded => Err(ServiceError::OrderAlreadySucceeded),
                Status::Cancelled => Ok(()), // Reducing database calls
                _ => {
                    let mut order_to_be_changed: OrderActiveModel = order.into();
                    order_to_be_changed.status = Set(Status::Cancelled);
                    order_to_be_changed.finished_at = Set(Some(Utc::now().naive_local()));
                    order_to_be_changed.save(connection).await?;
                    Ok(())
                }
            },
            None => Err(ServiceError::OrderNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn cancel_order_by_id<T>(order_id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match OrderEntity::find_by_id(order_id).one(connection).await? {
            Some(order) => match order.status {
                Status::Succeeded => Err(ServiceError::OrderAlreadySucceeded),
                Status::Cancelled => Ok(()),
                _ => {
                    let mut order_to_be_changed: OrderActiveModel = order.into();
                    order_to_be_changed.status = Set(Status::Cancelled);
                    order_to_be_changed.finished_at = Set(Some(Utc::now().naive_local()));
                    order_to_be_changed.save(connection).await?;
                    Ok(())
                }
            },
            None => Err(ServiceError::OrderNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn user_orders<T>(
        steam_id: i64,
        connection: &T,
    ) -> Result<Vec<OrderModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(OrderEntity::find()
            .filter(OrderColumn::SteamId.eq(steam_id))
            .all(connection)
            .await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn user_order<T>(
        parameters: impl Into<GetUserOrderParameters> + Debug,
        connection: &T,
    ) -> Result<Option<OrderModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let parameters = parameters.into();
        Ok(OrderEntity::find()
            .filter(
                OrderColumn::SteamId
                    .eq(parameters.steam_id)
                    .and(OrderColumn::Id.eq(parameters.order_id)),
            )
            .one(connection)
            .await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn finish_order_by_id<T>(
        order_id: i64,
        connection: &T,
    ) -> Result<OrderModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match OrderEntity::find_by_id(order_id).one(connection).await? {
            Some(order) => match order.status {
                Status::Succeeded => Err(ServiceError::OrderAlreadySucceeded),
                Status::Cancelled => Err(ServiceError::OrderAlreadyCanceled),
                _ => {
                    let mut order_to_be_changed: OrderActiveModel = order.into();
                    order_to_be_changed.status = Set(Status::Succeeded);
                    order_to_be_changed.finished_at = Set(Some(Utc::now().naive_local()));

                    let updated = order_to_be_changed
                        .save(connection)
                        .await?
                        .try_into_model()
                        .unwrap(); // Trust
                    Ok(updated)
                }
            },
            None => Err(ServiceError::OrderNotFound),
        }
    }

    pub async fn all_in_period<T>(
        period: (chrono::NaiveDateTime, chrono::NaiveDateTime),
        connection: &T,
    ) -> Result<Vec<OrderModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(OrderEntity::find()
            .filter(OrderColumn::FinishedAt.between(period.0, period.1))
            .order_by_desc(OrderColumn::FinishedAt)
            .all(connection)
            .await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn maybepayed<T>(
        parameters: impl Into<MayBePayedOrderParameters> + Debug,
        connection: &T,
    ) -> Result<OrderModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let params = parameters.into();

        match OrderEntity::find()
            .filter(
                OrderColumn::SteamId
                    .eq(params.steam_id)
                    .and(OrderColumn::Id.eq(params.order_id)),
            )
            .one(connection)
            .await?
        {
            Some(order) => match order.status {
                Status::Succeeded => Err(ServiceError::OrderAlreadySucceeded),
                Status::Cancelled => Err(ServiceError::OrderAlreadyCanceled),
                Status::Maybepayed => Ok(order),
                _ => {
                    let mut order_to_be_changed: OrderActiveModel = order.into();
                    order_to_be_changed.status = Set(Status::Maybepayed);

                    let updated = order_to_be_changed
                        .save(connection)
                        .await?
                        .try_into_model()
                        .unwrap(); // Trust
                    Ok(updated)
                }
            },
            None => Err(ServiceError::OrderNotFound),
        }
    }
}
