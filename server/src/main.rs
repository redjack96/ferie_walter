use axum::{routing::get, Router};
use crate::operazioni::elenco::get_all;
use crate::operazioni::hello_world::hello_world;

mod entity;
pub mod operazioni;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/getAll", get(get_all));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}