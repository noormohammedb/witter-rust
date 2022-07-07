use dotenv::dotenv;
use sqlx::pool::Pool;
use sqlx::query;
use sqlx::PgPool;
use tide;

#[async_std::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();
    pretty_env_logger::init();

    // dbg!(db_url);
    let db_url = std::env::var("DATABASE_URL").unwrap();

    let db_pool: PgPool = Pool::connect(&db_url).await.unwrap();

    let rows = query!("select (1) as id, 'Herp Derpinson' as name")
        .fetch_one(&db_pool)
        .await?;

    dbg!(rows);

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello, World!") });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum CustomError {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
