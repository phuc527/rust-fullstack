// routes/mod.rs ≈ routes/index.ts: ráp tất cả route vào 1 Router.
mod health;
mod seats;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

// with_state(state): gắn state vào router → handler lấy ra bằng extractor State(...).
// ≈ app.locals / middleware gắn req.db trong Express, nhưng có kiểu.
pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/health", get(health::health))
        .route("/seats", get(seats::list))
        // Cùng path, khác method: chain .delete() (≈ router.route(path).post().delete()).
        .route("/seats/{id}/book", post(seats::book).delete(seats::cancel));

    Router::new().nest("/api", api).with_state(state)
}
