use std::env;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn connect_db_pool() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been defined. Defined!");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;
    Ok(db)
}
