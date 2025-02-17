use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let rotes_hello = Router::new().route("/", get(|| async { "Hello, World!" }));

    //region: ----Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}\n", addr);
    //axum::Server::bind(&addr)
    //endregion: ----Start Server

}

