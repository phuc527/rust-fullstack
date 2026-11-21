use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::NaiveDateTime;
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr, time::Instant};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    status: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTaskRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateStatusRequest {
    status: String,
}

#[derive(Debug, Deserialize)]
struct TaskQuery {
    status: Option<String>,
    keyword: Option<String>,
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({"status": "ok", "message": "Task API is running"}))
}

async fn list_tasks(
    State(state): State<AppState>,
    Query(query): Query<TaskQuery>,
) -> Result<Json<Vec<Task>>, (StatusCode, Json<ApiError>)> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status, created_at, updated_at
        FROM tasks
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR description ILIKE '%' || $2 || '%')
        ORDER BY id DESC
        "#,
    )
    .bind(query.status)
    .bind(query.keyword)
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;
    Ok(Json(tasks))
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Task>, (StatusCode, Json<ApiError>)> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, status, created_at, updated_at FROM tasks WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    task.map(Json).ok_or_else(|| not_found("Task not found"))
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, Json<ApiError>)> {
    let title = payload.title.trim();
    validate_title(title)?;

    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description, status, created_at, updated_at
        "#,
    )
    .bind(title)
    .bind(payload.description)
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(task)))
}

async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, (StatusCode, Json<ApiError>)> {
    let title = payload.title.trim();
    validate_title(title)?;

    let task = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET title = $2, description = $3, updated_at = NOW()
        WHERE id = $1
        RETURNING id, title, description, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(title)
    .bind(payload.description)
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    task.map(Json).ok_or_else(|| not_found("Task not found"))
}

async fn update_task_status(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<Json<Task>, (StatusCode, Json<ApiError>)> {
    if !["pending", "in_progress", "completed"].contains(&payload.status.as_str()) {
        return Err(bad_request("Status must be pending, in_progress, or completed"));
    }

    let task = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET status = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING id, title, description, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(payload.status)
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    task.map(Json).ok_or_else(|| not_found("Task not found"))
}

async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<ApiError>)> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

    if result.rows_affected() == 0 {
        return Err(not_found("Task not found"));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn metrics_endpoint(handle: PrometheusHandle) -> impl IntoResponse {
    handle.render()
}

fn validate_title(title: &str) -> Result<(), (StatusCode, Json<ApiError>)> {
    if title.is_empty() {
        return Err(bad_request("Title cannot be empty"));
    }
    if title.chars().count() > 100 {
        return Err(bad_request("Title must be 100 characters or fewer"));
    }
    Ok(())
}

fn bad_request(message: &str) -> (StatusCode, Json<ApiError>) {
    (StatusCode::BAD_REQUEST, Json(ApiError { error: message.into() }))
}

fn not_found(message: &str) -> (StatusCode, Json<ApiError>) {
    (StatusCode::NOT_FOUND, Json(ApiError { error: message.into() }))
}

fn internal_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    warn!(error = %error, "database operation failed");
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
        error: "Internal server error".into()
    }))
}

async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id SERIAL PRIMARY KEY,
            title VARCHAR(100) NOT NULL,
            description TEXT,
            status VARCHAR(20) NOT NULL DEFAULT 'pending',
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

fn init_metrics() -> PrometheusHandle {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().expect("install Prometheus recorder");
    gauge!("task_api_build_info", "version" => env!("CARGO_PKG_VERSION")).set(1.0);
    handle
}

fn init_logging() {
    let filter = env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=info,sqlx=warn".into());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_logging();
    let prometheus_handle = init_metrics();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "3000".into())
        .parse::<u16>().expect("APP_PORT must be a valid port");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    init_db(&db).await.expect("Failed to initialize database");

    let state = AppState { db };
    let metrics_handle_for_route = prometheus_handle.clone();

    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route("/api/tasks/{id}", get(get_task).put(update_task).delete(delete_task))
        .route("/api/tasks/{id}/status", axum::routing::patch(update_task_status))
        .with_state(state);

    let app = Router::new()
        .route("/metrics", get(move || metrics_endpoint(metrics_handle_for_route.clone())))
        .merge(api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Basic HTTP metrics middleware is implemented below via a route-level fallback wrapper.
    // Metrics are also emitted for task handlers; see instrumented routes.
    let app = instrument_requests(app);

    let addr = SocketAddr::new(host.parse().expect("APP_HOST must be IP address"), port);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind failed");
    info!(address = %addr, "Task API starting");
    axum::serve(listener, app).await.expect("server failed");
}

// Wrap each incoming request and record total count + latency.
// Use URI path only for routes without IDs; normalize task ID paths to avoid metric cardinality explosion.
fn instrument_requests(app: Router) -> Router {
    use axum::{body::Body, http::Request, middleware::Next};
    async fn middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
        let method = req.method().to_string();
        let raw_path = req.uri().path().to_string();
        let route_label = if raw_path.starts_with("/api/tasks/") {
            if raw_path.ends_with("/status") { "/api/tasks/{id}/status" } else { "/api/tasks/{id}" }
        } else {
            raw_path.as_str()
        };
        let started = Instant::now();
        let response = next.run(req).await;
        let status = response.status().as_u16().to_string();
        counter!("http_requests_total", "method" => method.clone(), "route" => route_label.to_string(), "status" => status).increment(1);
        histogram!("http_request_duration_seconds", "method" => method, "route" => route_label.to_string()).record(started.elapsed().as_secs_f64());
        response
    }
    app.layer(axum::middleware::from_fn(middleware))
}
