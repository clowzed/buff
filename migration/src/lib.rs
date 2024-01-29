pub use sea_orm_migration::prelude::*;

mod m20240116_115527_create_users;
mod m20240116_134755_create_blacklist;
mod m20240116_141203_create_admins;
mod m20240116_222821_create_reviews;
mod m20240117_125548_create_video_reviews;
mod m20240117_153036_create_orders;
mod m20240119_082815_create_currency_rates;
mod m20240129_092330_create_social;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240116_115527_create_users::Migration),
            Box::new(m20240116_134755_create_blacklist::Migration),
            Box::new(m20240116_141203_create_admins::Migration),
            Box::new(m20240116_222821_create_reviews::Migration),
            Box::new(m20240117_125548_create_video_reviews::Migration),
            Box::new(m20240117_153036_create_orders::Migration),
            Box::new(m20240119_082815_create_currency_rates::Migration),
            Box::new(m20240129_092330_create_social::Migration),
        ]
    }
}
