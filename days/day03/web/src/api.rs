// Lớp gọi HTTP (≈ src/api/seats.ts). Component không gọi fetch trực tiếp.
use gloo_net::http::Request;

use crate::models::Seat;

// `?` = Err thì return ngay (≈ throw nhưng hiện rõ trong kiểu trả về).
// map_err: đổi lỗi gloo_net → String để hiển thị (và Clone được).
pub async fn fetch_seats() -> Result<Vec<Seat>, String> {
    let res = Request::get("/api/seats")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    res.json::<Vec<Seat>>().await.map_err(|e| e.to_string())
}

// POST /api/seats/{id}/book. &str = MƯỢN chuỗi (không lấy quyền sở hữu) → caller vẫn dùng được id.
pub async fn book_seat(id: &str) -> Result<Seat, String> {
    let res = Request::post(&format!("/api/seats/{id}/book"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    // Giống fetch của JS: 4xx/5xx KHÔNG phải lỗi mạng → tự kiểm tra status.
    match res.status() {
        200 => res.json::<Seat>().await.map_err(|e| e.to_string()),
        409 => Err("Ghế đã có người đặt".into()),
        404 => Err("Không tìm thấy ghế".into()),
        s => Err(format!("Lỗi HTTP {s}")),
    }
}

// DELETE /api/seats/{id}/book — gỡ đặt.
pub async fn cancel_seat(id: &str) -> Result<Seat, String> {
    let res = Request::delete(&format!("/api/seats/{id}/book"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    match res.status() {
        200 => res.json::<Seat>().await.map_err(|e| e.to_string()),
        409 => Err("Ghế này chưa được đặt".into()),
        404 => Err("Không tìm thấy ghế".into()),
        s => Err(format!("Lỗi HTTP {s}")),
    }
}
