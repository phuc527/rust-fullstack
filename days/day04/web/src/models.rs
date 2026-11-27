use serde::Deserialize;

// Cùng enum với api. Day 6 gom vào crate `shared`.
// JSON "held" → SeatStatus::Held. Chuỗi lạ ("sold") → lỗi parse, không lọt vào UI.
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeatStatus {
    Free,
    Held,
    Booked,
}

impl SeatStatus {
    // `&'static str`: chuỗi nằm sẵn trong binary, không cấp phát. Dùng làm CSS class.
    pub fn class(self) -> &'static str {
        match self {
            SeatStatus::Free => "free",
            SeatStatus::Held => "held",
            SeatStatus::Booked => "booked",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SeatStatus::Free => "Trống",
            SeatStatus::Held => "Đang giữ",
            SeatStatus::Booked => "Đã đặt",
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Seat {
    pub id: String,
    pub row: char,
    pub col: u8,
    pub status: SeatStatus,
}
