use sea_orm::Database;
use dotenv::dotenv;
use std::env;

pub async fn establish_connection() -> Result<Database, DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    Database::connect(&database_url).await
}
