use dotenv::dotenv;
use serde_json::json;
use sqlx::pool::Pool;
use sqlx::query;
use sqlx::PgPool;
use tide::Request;
use tide::Server;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() {
    let app = my_server().await;

    app.listen("127.0.0.1:8080").await.unwrap();
}

async fn my_server() -> Server<ServerState> {
    dotenv().ok();
    pretty_env_logger::init();
    // dbg!(db_url);
    let db_url = std::env::var("DATABASE_URL").unwrap();

    let db_pool: PgPool = Pool::connect(&db_url).await.unwrap();

    let mut app: Server<ServerState> = Server::with_state(ServerState { db_pool });
    /*
    let rows = query!("select (1) as id, 'Herp Derpinson' as name")
        .fetch_one(&db_pool)
        .await
        .unwrap();
    dbg!(rows);
     */

    app.at("/").get(|req: Request<ServerState>| async move {
        // app.at("/").get(|_| async move {
        // /*
        let db_pool = &req.state().db_pool;

        let rows = query!("select (1) as id, 'Herp Derpinson' as name")
            .fetch_one(db_pool)
            .await
            .unwrap();

        // dbg!(&rows);

        let my_json = json!([rows.id.unwrap(), rows.name.unwrap()]);
        Ok(my_json)

        // let my_json = json!(rows);

        // Ok("Hello, World!")
    });

    app
}

#[derive(Debug, Clone)]
struct ServerState {
    db_pool: PgPool,
}

/*
#[derive(thiserror::Error, Debug)]
enum CustomError {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
*/

