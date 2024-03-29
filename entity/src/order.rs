//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use super::sea_orm_active_enums::Status;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub payment_method: String,
    pub status: Status,
    #[sea_orm(column_type = "Text", nullable)]
    pub requisites: Option<String>,
    pub created_at: DateTime,
    pub finished_at: Option<DateTime>,
    pub steam_id: i64,
    pub moderator_id: Option<i64>,
    pub amount: Decimal,
    pub fixed_currency_rate: Decimal,
    #[sea_orm(column_type = "Text")]
    pub currency_symbol: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::admin::Entity",
        from = "Column::ModeratorId",
        to = "super::admin::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Admin,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SteamId",
        to = "super::user::Column::SteamId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::admin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Admin.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
