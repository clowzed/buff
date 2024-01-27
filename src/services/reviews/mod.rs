use crate::errors::AppError;
use crate::errors::Details;
use entity::review::ActiveModel as ReviewActiveModel;
use entity::review::Entity as ReviewEntity;
use entity::review::Model as ReviewModel;
use entity::user::Entity as UserEntity;
use entity::video_review::ActiveModel as VideoReviewActiveModel;
use entity::video_review::Column::Url as VideoReviewColumnUrl;
use entity::video_review::Entity as VideoReviewEntity;
use entity::video_review::Model as VideoReviewModel;
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
    pub async fn users_all<T>(connection: &T) -> Result<Vec<ReviewModel>, ServiceError>
    where
        T: ConnectionTrait + TransactionTrait,
    {
        Ok(ReviewEntity::find().all(connection).await?)
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
}
