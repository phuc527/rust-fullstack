// `use` ≈ `import` trong Node. Router = app của Express, get = app.get
use axum::{Router, routing::get};

// Rust không có event loop sẵn như Node. #[tokio::main] biến main thành async
// và khởi động runtime Tokio (≈ libuv của Node).
#[tokio::main]
async fn main() {
    // ≈ const app = express(); app.get("/api/health", health)
    let app = Router::new().route("/api/health", get(health));

    // 0.0.0.0 để container khác (nginx) gọi vào được; 127.0.0.1 chỉ nội bộ container.
    // `.await` ≈ await; `.unwrap()` = "lỗi thì crash" (OK cho day 1, Day 5 sẽ xử lý đúng cách).
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("api on :3000");
    axum::serve(listener, app).await.unwrap();
}

// Handler: trả về &'static str = chuỗi cố định nằm sẵn trong binary.
// Axum tự biến nó thành response 200 text/plain (≈ res.send("ok")).
async fn health() -> &'static str {
    "ok"
}
