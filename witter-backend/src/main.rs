use dotenv::dotenv;
use tide;

#[async_std::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();

    // let db_url = std::env::var("DATABASE_URL").unwrap();
    // dbg!(db_url);

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
}
