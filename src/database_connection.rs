use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;
use dotenvy::dotenv;
use std::env;

pub type DbPool = Arc<Pool<ConnectionManager<PgConnection>>>;

// Function to create the database pool
pub fn create_db_pool() -> DbPool {
    dotenv().ok(); // Load environment variables

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(5) // Adjust pool size as needed
        .build(manager)
        .expect("Failed to create database pool.");

    Arc::new(pool)
}
