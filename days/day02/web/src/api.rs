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
