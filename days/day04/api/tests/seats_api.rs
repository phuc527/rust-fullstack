// Integration test cho HTTP (≈ supertest). Gọi router trực tiếp, không mở cổng mạng.
use api::{routes, state};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tower::ServiceExt; // .oneshot(): gửi 1 request vào router

async fn call(app: &axum::Router, method: Method, uri: &str) -> StatusCode {
    let req = Request::builder().method(method).uri(uri).body(Body::empty()).unwrap();
    // Router clone rẻ (bên trong là Arc) → dùng chung state giữa các request.
    app.clone().oneshot(req).await.unwrap().status()
}

#[tokio::test]
async fn book_then_cancel() {
    let app = routes::router(state::new_state());
    assert_eq!(call(&app, Method::POST, "/api/seats/A1/book").await, StatusCode::OK);
    assert_eq!(call(&app, Method::POST, "/api/seats/A1/book").await, StatusCode::CONFLICT);
    assert_eq!(call(&app, Method::DELETE, "/api/seats/A1/book").await, StatusCode::OK);
    assert_eq!(call(&app, Method::DELETE, "/api/seats/A1/book").await, StatusCode::CONFLICT);
}

#[tokio::test]
async fn unknown_seat_is_404() {
    let app = routes::router(state::new_state());
    assert_eq!(call(&app, Method::POST, "/api/seats/Z9/book").await, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn held_seat_is_409() {
    let app = routes::router(state::new_state());
    assert_eq!(call(&app, Method::POST, "/api/seats/C5/book").await, StatusCode::CONFLICT);
}
