// main.rs = entrypoint, CHỈ khởi động server (≈ server.js / index.ts).
use api::{config::Config, routes, state};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    // State tạo 1 lần lúc khởi động, router giữ nó và đưa cho từng handler.
    // ponytail: data trong RAM, restart là mất — Day 8 chuyển sang Postgres.
    let app = routes::router(state::new_state());

    let listener = tokio::net::TcpListener::bind(config.addr())
        .await
        .expect("không bind được cổng");
    println!("api on {}", config.addr());

    axum::serve(listener, app).await.unwrap();
}
