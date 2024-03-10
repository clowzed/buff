use crate::errors::{AppError, Details};
use entity::{
    review::{
        ActiveModel as ReviewActiveModel, Column as ReviewColumn, Entity as ReviewEntity,
        Model as ReviewModel,
    },
    user::Entity as UserEntity,
    video_review::{
        ActiveModel as VideoReviewActiveModel, Column::Url as VideoReviewColumnUrl,
        Entity as VideoReviewEntity, Model as VideoReviewModel,
    },
};
use migration::Query;
use sea_orm::{prelude::*, Condition, QueryOrder, QuerySelect, Set, TransactionTrait};
use std::fmt::Debug;

#[allow(dead_code)]
pub struct Service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error("User with id = {0} was not found")]
    UserNotFound(i64),
    #[error("Star check failed. Expected amount of stars between 0 and 5")]
    StarsCheckFailed,
    #[error("Url already exists")]
    UrlAlreadyExists,
    #[error("Video review with provided id was not found")]
    VideoReviewIdNotFound,
    #[error("Review with provided id was not found")]
    ReviewIdNotFound,
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbErr(cause) => AppError::InternalServerError(Box::new(cause)),
            ServiceError::UserNotFound(id) => AppError::UserWasNotFound(id),
            ServiceError::StarsCheckFailed => AppError::BadRequest(Details {
                details: value.to_string(),
            }),
            ServiceError::UrlAlreadyExists => AppError::UrlAlreadyExists,
            ServiceError::VideoReviewIdNotFound => AppError::VideoReviewIdNotFound,
            ServiceError::ReviewIdNotFound => AppError::ReviewWasNotFound,
        }
    }
}

#[derive(Debug)]
pub struct AddReviewParameters {
    pub steam_id: i64,
    pub review: String,
    pub stars: i16,
}

#[derive(Debug)]
pub struct AddVideoReviewParameters {
    pub url: String,
    pub avatar: String,
    pub name: String,
    pub subscribers: String,
}

#[derive(Debug)]
pub struct UpdateVideoReviewParameters {
    pub id: i64,
    pub url: Option<String>,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub subscribers: Option<String>,
}

impl UpdateVideoReviewParameters {
    pub fn empty(&self) -> bool {
        self.url.is_none()
            && self.avatar.is_none()
            && self.name.is_none()
            && self.subscribers.is_none()
    }
}
impl Service {
    #[tracing::instrument(skip(connection))]
    pub async fn add_users_review<T>(
        parameters: impl Into<AddReviewParameters> + Debug,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let review: AddReviewParameters = parameters.into();

        match UserEntity::find_by_id(review.steam_id)
            .one(connection)
            .await?
        {
            Some(_) => {
                if review.stars > 5 || review.stars < 0 {
                    return Err(ServiceError::StarsCheckFailed);
                }

                let review_to_be_inserted = ReviewActiveModel {
                    steam_id: Set(review.steam_id),
                    review: Set(review.review),
                    stars: Set(review.stars),
                    ..Default::default()
                };

                review_to_be_inserted.save(connection).await?;
                Ok(())
            }
            None => Err(ServiceError::UserNotFound(review.steam_id)),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn users_all<T>(
        limit: u64,
        offset: u64,
        connection: &T,
    ) -> Result<Vec<ReviewModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(ReviewEntity::find()
            .filter(
                Condition::any().add(
                    ReviewColumn::SteamId.not_in_subquery(
                        Query::select()
                            .column(entity::blacklisted::Column::SteamId)
                            .from(entity::blacklisted::Entity)
                            .to_owned(),
                    ),
                ),
            )
            .order_by_desc(ReviewColumn::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(connection)
            .await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn videos_all<T>(connection: &T) -> Result<Vec<VideoReviewModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(VideoReviewEntity::find().all(connection).await?)
    }

    #[tracing::instrument(skip(connection))]
    pub async fn add_video_review<T>(
        parameters: impl Into<AddVideoReviewParameters> + Debug,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let review: AddVideoReviewParameters = parameters.into();

        match VideoReviewEntity::find()
            .filter(VideoReviewColumnUrl.eq(&review.url))
            .one(connection)
            .await?
        {
            None => {
                let review_to_be_inserted = VideoReviewActiveModel {
                    url: Set(review.url),
                    avatar: Set(review.avatar),
                    name: Set(review.name),
                    subscribers: Set(review.subscribers),
                    ..Default::default()
                };

                review_to_be_inserted.save(connection).await?;
                Ok(())
            }
            Some(_) => Err(ServiceError::UrlAlreadyExists),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn remove_video_review<T>(id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match VideoReviewEntity::find_by_id(id).one(connection).await? {
            Some(video) => {
                let video_to_be_removed: VideoReviewActiveModel = video.into();
                video_to_be_removed.delete(connection).await?;
                Ok(())
            }
            None => Err(ServiceError::VideoReviewIdNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn remove_review<T>(id: i64, connection: &T) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        match ReviewEntity::find_by_id(id).one(connection).await? {
            Some(review) => {
                let review_to_be_removed: ReviewActiveModel = review.into();
                review_to_be_removed.delete(connection).await?;
                Ok(())
            }
            None => Err(ServiceError::ReviewIdNotFound),
        }
    }

    #[tracing::instrument(skip(connection))]
    pub async fn update_video_review<T>(
        parameters: impl Into<UpdateVideoReviewParameters> + Debug,
        connection: &T,
    ) -> Result<(), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        let provided = parameters.into();

        if provided.empty() {
            return Ok(());
        }

        match VideoReviewEntity::find_by_id(provided.id)
            .one(connection)
            .await?
        {
            Some(video) => {
                let mut video_to_be_updated: VideoReviewActiveModel = video.into();

                if let Some(name) = provided.name {
                    video_to_be_updated.name = Set(name);
                }
                if let Some(avatar) = provided.avatar {
                    video_to_be_updated.avatar = Set(avatar);
                }
                if let Some(url) = provided.url {
                    video_to_be_updated.url = Set(url);
                }

                if let Some(subscribers) = provided.subscribers {
                    video_to_be_updated.subscribers = Set(subscribers);
                }

                video_to_be_updated.update(connection).await?;
                Ok(())
            }
            None => Err(ServiceError::VideoReviewIdNotFound),
        }
    }

    pub async fn count<T>(connection: &T) -> Result<(u64, u64), ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok((
            ReviewEntity::find()
                .filter(
                    Condition::any().add(
                        ReviewColumn::SteamId.not_in_subquery(
                            Query::select()
                                .column(entity::blacklisted::Column::SteamId)
                                .from(entity::blacklisted::Entity)
                                .to_owned(),
                        ),
                    ),
                )
                .count(connection)
                .await?,
            VideoReviewEntity::find()
                .filter(
                    Condition::any().add(
                        ReviewColumn::SteamId.not_in_subquery(
                            Query::select()
                                .column(entity::blacklisted::Column::SteamId)
                                .from(entity::blacklisted::Entity)
                                .to_owned(),
                        ),
                    ),
                )
                .count(connection)
                .await?,
        ))
    }

    pub async fn five_stars<T>(connection: &T) -> Result<Vec<ReviewModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(ReviewEntity::find()
            .filter(ReviewColumn::Stars.eq(5))
            .filter(
                Condition::any().add(
                    ReviewColumn::SteamId.not_in_subquery(
                        Query::select()
                            .column(entity::blacklisted::Column::SteamId)
                            .from(entity::blacklisted::Entity)
                            .to_owned(),
                    ),
                ),
            )
            .order_by_desc(ReviewColumn::CreatedAt)
            .all(connection)
            .await?)
    }
}
