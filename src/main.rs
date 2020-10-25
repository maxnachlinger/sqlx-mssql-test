use sqlx::mssql::{MssqlPool};

// assuming a local mssql instance started via:
// docker run -e 'ACCEPT_EULA=Y' -e 'SA_PASSWORD=<YourStrong@Passw0rd>' -p 1433:1433 -d mcr.microsoft.com/mssql/server:2017-latest

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let dsn = "mssql://127.0.0.1:1433;database=master;user=SA;password=<YourStrong@Passw0rd>s";
    let pool = MssqlPool::connect(&dsn).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
