use dotenv::dotenv;
use serde::Serialize;
use serde_json::json;
use sqlx::pool::Pool;
use sqlx::PgPool;
use sqlx::{query, query_as};
use tide::Request;
use tide::Server;
use uuid::Uuid;

// use http_types::Body;
// use tide::http::StatusCode;
// use tide::Response;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() {
    let app = my_server().await;

    app.listen("127.0.0.1:8080").await.unwrap();
}

// #[cfg(not(test))]
async fn make_db_poo() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    Pool::connect(&db_url).await.unwrap()
}

async fn my_server() -> Server<ServerState> {
    dotenv().ok();
    pretty_env_logger::init();

    let db_pool = make_db_poo().await;
    let mut app: Server<ServerState> = Server::with_state(ServerState { db_pool });

    app.at("/").get(|req: Request<ServerState>| async move {
        let db_pool = &req.state().db_pool;

        let rows = query!("select count(*) from users")
            .fetch_one(db_pool)
            .await
            .unwrap();

        dbg!(&rows);

        // let my_json = json!([rows.id.unwrap(), rows.name.unwrap()]);
        // Ok(Response::new(StatusCode::Ok).set_body(Body::from_json(&my_json)))

        let my_json = json!(["foo", "bar"]);

        // Ok(Response::new(StatusCode::Ok).set_body(Body::from_json(&my_json)))
        Ok(my_json)

        // Ok(Response::new(StatusCode::Ok).bod)
        // Ok(Response::new(StatusCode::Ok).body_json(&rows)?)

        // let my_json = json!(rows);

        // Ok("Hello, World!")
    });

    app.at("/users")
        .get(|req: Request<ServerState>| async move {
            let db_pool = &req.state().db_pool;

            let rows = query_as!(User, "select id, username from users")
                .fetch_all(db_pool)
                .await
                .unwrap();

            let user_list_json = json!(rows);

            // Ok(Response::new(StatusCode::Ok).set_body(Body::from_json(&user_list_json)?))
            Ok(user_list_json)
        });

    app
}

#[derive(Debug, Clone)]
struct ServerState {
    db_pool: PgPool,
}

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    username: String,
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
