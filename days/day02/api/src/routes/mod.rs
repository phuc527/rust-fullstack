// routes/mod.rs ≈ routes/index.ts: ráp tất cả route vào 1 Router.
mod health;
mod seats;

use axum::{Router, routing::get};

// nest("/api", ...) ≈ app.use('/api', apiRouter) trong Express.
pub fn router() -> Router {
    let api = Router::new()
        .route("/health", get(health::health))
        .route("/seats", get(seats::list));

    Router::new().nest("/api", api)
}
