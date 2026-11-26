use serde::Deserialize;

// Giống bên api nhưng Deserialize (JSON → struct).
// Đang trùng code với api — Day 6 gom vào crate `shared`.
// JSON thiếu field / sai kiểu → lỗi ngay khi parse (JS thì `undefined` âm thầm).
#[derive(Deserialize, Clone, Debug)]
pub struct Seat {
    pub id: String,
    pub row: char,
    pub col: u8,
    pub booked: bool,
}
