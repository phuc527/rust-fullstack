use axum::Json;

use crate::models::Seat; // crate:: = gốc crate hiện tại (≈ import từ '@/models')

// Handler mỏng: chỉ nhận request → gọi model → trả JSON (≈ controller).
// Json(...) tự serialize + set content-type: application/json ≈ res.json(...)
pub async fn list() -> Json<Vec<Seat>> {
    Json(Seat::grid())
}
