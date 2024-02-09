use std::fmt::Debug;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use entity::{
    admin::{
        ActiveModel as AdminActiveModel, Column as AdminColumn, Entity as AdminEntity,
        Model as AdminModel,
    },
    order::{
        ActiveModel as OrderActiveModel, Column as OrderColumn, Entity as OrderEntity,
        Model as OrderModel,
    },
    sea_orm_active_enums::{Role, Status},
};
use rand_core::OsRng;
use sea_orm::{prelude::*, Set, TransactionTrait};

use crate::errors::AppError;

#[allow(dead_code)]
pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Provided login already exists")]
    LoginAlreadyExists,
    #[error(transparent)]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("Admin or moderator was not found")]
    AdminNotFound,
    #[error("Moderator has admin role")]
    ModeratorIsAdmin,
    #[error("Moderator has already been assigned to this order")]
    ModeratorAlreadyAssigned,
    #[error("Order was not found")]
    OrderWasNotFound,
    #[error("Moderator is not assigned to this order")]
    ModeratorNotAssigned,
    #[error("Another moderator has been assigned to this order")]
    AnotherModeratorAlreadyAssigned,
    #[error("Order is completed or cancelled")]
    OrderIsCompletedOrCancelled,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::LoginAlreadyExists => AppError::LoginOccupied,
            ServiceError::PasswordHashError(cause) => {
                AppError::InternalServerError(Box::new(cause))
            }
            ServiceError::AdminNotFound => AppError::AdminNotFound,
            ServiceError::ModeratorIsAdmin => AppError::ModeratorIsAdmin,
            ServiceError::ModeratorAlreadyAssigned => AppError::ModeratorAlreadyAssigned,
            ServiceError::OrderWasNotFound => AppError::OrderWasNotFound,
            ServiceError::ModeratorNotAssigned => AppError::ModeratorNotAssigned,
            ServiceError::AnotherModeratorAlreadyAssigned => AppError::ModeratorAlreadyAssigned,
            ServiceError::OrderIsCompletedOrCancelled => AppError::OrderIsCompletedOrCancelled,
        }
    }
}

pub struct CreateModeratorParameters {
    pub login: String,
    pub password: String,
}

impl Debug for CreateModeratorParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateModeratorParameters")
            .field("login", &self.login)
            .finish()
    }
}

#[derive(Debug)]
pub struct AssignModeratorParameters {
    pub moderator_id: i64,
    pub order_id: i64,
}

#[derive(Debug)]
pub struct UnassignModeratorParameters {
    pub moderator_id: i64,
    pub order_id: i64,
}

impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn create_moderator<T>(
        params: CreateModeratorParameters,
        connection: &T,
    ) -> Result<AdminModel, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match AdminEntity::find()
            .filter(AdminColumn::Login.eq(&params.login))
            .one(connection)
            .await?
        {
            Some(_) => Err(ServiceError::LoginAlreadyExists),
            None => {
                let salt = SaltString::generate(&mut OsRng);

                let hashed_password = Argon2::default()
                    .hash_password(params.password.as_bytes(), &salt)?
                    .to_string();

                let admin_to_be_inserted = AdminActiveModel {
                    login: Set(params.login),
                    password: Set(hashed_password),
                    role: Set(Role::Moderator),
                    ..Default::default()
                };

                Ok(AdminEntity::insert(admin_to_be_inserted)
                    .exec_with_returning(connection)
                    .await?)
            }
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn delete_moderator<T>(moderator_id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match AdminEntity::find_by_id(moderator_id)
            .one(connection)
            .await?
        {
            Some(admin_or_moderator) => match admin_or_moderator.role {
                Role::Admin => Err(ServiceError::ModeratorIsAdmin),
                Role::Moderator => {
                    OrderEntity::update_many()
                        .col_expr(OrderColumn::ModeratorId, Expr::value::<Option<i64>>(None))
                        .filter(
                            OrderColumn::ModeratorId
                                .eq(moderator_id)
                                .and(OrderColumn::Status.eq(Status::Created)),
                        )
                        .exec(connection)
                        .await?;

                    let moderator_to_be_deleted: AdminActiveModel = admin_or_moderator.into();
                    moderator_to_be_deleted.delete(connection).await?;
                    Ok(())
                }
            },
            None => Err(ServiceError::AdminNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn assign_moderator<T>(
        parameters: AssignModeratorParameters,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let moderator = match AdminEntity::find_by_id(parameters.moderator_id)
            .one(connection)
            .await?
        {
            Some(moderator) => match moderator.role {
                Role::Admin => Err(ServiceError::ModeratorIsAdmin),
                Role::Moderator => Ok(moderator),
            },
            None => Err(ServiceError::AdminNotFound),
        }?;

        let order = match OrderEntity::find_by_id(parameters.order_id)
            .one(connection)
            .await?
        {
            Some(order) => match order.moderator_id {
                Some(_) => Err(ServiceError::ModeratorAlreadyAssigned),
                None => match order.status {
                    Status::Succeeded | Status::Cancelled => {
                        Err(ServiceError::OrderIsCompletedOrCancelled)
                    }
                    _ => Ok(order),
                },
            },
            None => Err(ServiceError::OrderWasNotFound),
        }?;

        let mut order_to_be_updated: OrderActiveModel = order.into();

        order_to_be_updated.moderator_id = Set(Some(moderator.id));
        order_to_be_updated.update(connection).await?;
        Ok(())
    }

    #[tracing::instrument(skip(connection))]
    pub async fn unassign_moderator<T>(
        parameters: UnassignModeratorParameters,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let order = match OrderEntity::find_by_id(parameters.order_id)
            .one(connection)
            .await?
        {
            Some(order) => match order.moderator_id {
                Some(id) => match id == parameters.moderator_id {
                    true => match order.status {
                        Status::Succeeded | Status::Cancelled => {
                            Err(ServiceError::OrderIsCompletedOrCancelled)
                        }
                        _ => Ok(order),
                    },
                    false => Err(ServiceError::AnotherModeratorAlreadyAssigned),
                },
                None => Err(ServiceError::ModeratorNotAssigned),
            },
            None => Err(ServiceError::AdminNotFound),
        }?;

        let mut order_to_be_updated: OrderActiveModel = order.into();

        order_to_be_updated.moderator_id = Set(None);
        order_to_be_updated.update(connection).await?;
        Ok(())
    }

    pub async fn moderators<T>(connection: &T) -> Result<Vec<AdminModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(AdminEntity::find()
            .filter(AdminColumn::Role.eq(Role::Moderator))
            .all(connection)
            .await?)
    }

    pub async fn moderators_orders<T>(
        moderator_id: i64,
        connection: &T,
    ) -> Result<Vec<OrderModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let moderator = match AdminEntity::find_by_id(moderator_id)
            .one(connection)
            .await?
        {
            Some(moderator_or_admin) => match moderator_or_admin.role {
                Role::Admin => Err(ServiceError::ModeratorIsAdmin),
                Role::Moderator => Ok(moderator_or_admin),
            },
            None => Err(ServiceError::AdminNotFound),
        }?;

        Ok(moderator.find_related(OrderEntity).all(connection).await?)
    }
}
