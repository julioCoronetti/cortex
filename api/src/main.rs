mod routes;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes::routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("cortex-api listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
