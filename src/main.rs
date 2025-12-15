use axum::Router;
use axum::routing::get;
use axum::routing::post;
use todos::db;
use todos::handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = db::connect_db().await?;

    let app: Router<_> = Router::new()
        .route("/todos", post(create_todo).get(get_all_todos))
        .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(database);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
