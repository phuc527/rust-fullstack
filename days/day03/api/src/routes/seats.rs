use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{models::Seat, state::AppState};

// Extractor = tham số handler, Axum tự lấy từ request (≈ req.params, req.app.locals).
// State(state): pattern destructuring — mở hộp State lấy AppState ra.
pub async fn list(State(state): State<AppState>) -> Json<Vec<Seat>> {
    // lock() trả MutexGuard; guard bị drop cuối câu lệnh → tự mở khóa (không cần unlock()).
    // .clone(): trả BẢN COPY ra ngoài, dữ liệu gốc vẫn thuộc state.
    Json(state.lock().unwrap().clone())
}

// POST /api/seats/{id}/book
// Ok → 200 + JSON; Err(StatusCode) → trả status đó (Day 5 làm lỗi đẹp hơn).
pub async fn book(
    State(state): State<AppState>,
    Path(id): Path<String>, // ≈ req.params.id
) -> Result<Json<Seat>, StatusCode> {
    // `mut`: biến mặc định BẤT BIẾN, muốn sửa phải ghi mut (ngược với JS: let mặc định sửa được).
    let mut seats = state.lock().unwrap();

    // iter_mut(): duyệt bằng &mut Seat — mượn để SỬA tại chỗ.
    // ok_or(...)? : None → trả 404 ngay (≈ if (!seat) return res.sendStatus(404)).
    let seat = seats
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if !seat.book() {
        return Err(StatusCode::CONFLICT); // 409: ghế đã có người đặt
    }
    Ok(Json(seat.clone()))
} // `seats` (guard) bị drop ở đây → mở khóa

// DELETE /api/seats/{id}/book — gỡ đặt. 409 nếu ghế đang trống.
// ponytail: chưa có user nên ai cũng gỡ được; Day auth sẽ chỉ cho chủ ghế gỡ.
pub async fn cancel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Seat>, StatusCode> {
    let mut seats = state.lock().unwrap();
    let seat = seats
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if !seat.cancel() {
        return Err(StatusCode::CONFLICT);
    }
    Ok(Json(seat.clone()))
}
