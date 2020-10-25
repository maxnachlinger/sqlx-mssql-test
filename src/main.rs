use sqlx::mssql::{MssqlPool, MssqlConnectOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let options = MssqlConnectOptions::new()
        .host("127.0.0.1")
        .port(1433)
        .database("test")
        .username("test")
        .password("test_p4ssword");

    let pool = MssqlPool::connect_with(options).await?;

    let row: (i32,String) = sqlx::query_as("SELECT id, name FROM test_table")
        .fetch_one(&pool).await?;

    dbg!(row);

    Ok(())
}
