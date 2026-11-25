// main.rs = entrypoint, CHỈ khởi động server (≈ server.js / index.ts).
// Logic thật nằm trong lib (lib.rs) để test và tái sử dụng được.
use api::{config::Config, routes};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let listener = tokio::net::TcpListener::bind(config.addr())
        .await
        .expect("không bind được cổng");
    println!("api on {}", config.addr());

    axum::serve(listener, routes::router()).await.unwrap();
}
