use std::env;

use sea_orm::{Database, DatabaseConnection, DbErr};

use sea_orm::ConnectOptions;
use std::time::Duration;
use tracing::log::LevelFilter;
use color_eyre::Result;


// /// Function to create the database connection pool
pub async fn create_db_pool() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info);

    Database::connect(opt).await
}


// // export DATABASE_URL=postgres://postgres:md@localhost/toolshub
// // cargo run
