use sea_orm::*;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";
const DB_NAME: &str = "postgres";

pub(super) async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db_conn = Database::connect(&url).await?;

    Ok(db_conn)
}