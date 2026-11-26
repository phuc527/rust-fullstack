use std::sync::{Arc, Mutex};

use crate::models::Seat;

// State dùng chung cho mọi request (≈ biến global `let seats = [...]` trong Express).
// Rust không cho share dữ liệu mutable giữa các thread tùy tiện, nên phải bọc:
//   Arc   = con trỏ đếm tham chiếu, clone() chỉ tăng bộ đếm (rẻ) → nhiều handler cùng giữ
//   Mutex = khóa: tại 1 thời điểm chỉ 1 handler được sửa → không race condition
// Node chạy 1 thread nên không cần; Tokio chạy NHIỀU thread nên compiler bắt buộc.
pub type AppState = Arc<Mutex<Vec<Seat>>>;

pub fn new_state() -> AppState {
    Arc::new(Mutex::new(Seat::grid()))
}
