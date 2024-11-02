// use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
use leptos::server_fn::ServerFnError;
use sqlx::{Connection, SqliteConnection, Sqlite};
use sqlx::sqlite::SqlitePool;
use sqlx::migrate::MigrateDatabase;

pub static CONNECTOR: &'static str = env!("DATABASE_CONNECTOR");

pub async fn conn() -> Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect(CONNECTOR).await?)
}

pub async fn make_migration() -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(CONNECTOR).await? {
        Sqlite::create_database(CONNECTOR).await?;
    }

    let pool = SqlitePool::connect(&CONNECTOR).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}