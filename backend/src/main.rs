#![allow(unused)]

use axum::routing::get;
use axum::{Router, ServiceExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let routes_hello = Router::new().route("/hello", get(|| async { "Hello world!" }));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    info!(
        "Listening on port {:?}",
        listener.local_addr().unwrap().port()
    );

    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}
