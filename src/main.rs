use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let rotes_hello = Router::new().route("/", get(|| async { "Hello, World!" }));

    //region: ----Start Server
    let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}\n", addr.local_addr().unwrap());
    axum::serve(addr, rotes_hello).await.unwrap();
    //endregion: ----Start Server

}

