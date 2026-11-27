// Integration test (≈ __tests__/models.test.ts). Chỉ dùng API `pub` của lib.
use web::models::{Seat, SeatStatus};

#[test]
fn parse_seat_json() {
    let s: Seat = serde_json::from_str(r#"{"id":"A1","row":"A","col":1,"status":"held"}"#).unwrap();
    assert_eq!(s.status, SeatStatus::Held);
}

#[test]
fn unknown_status_is_error() {
    // TS union chỉ check lúc compile; JSON "sold" vẫn lọt vào runtime. Rust chặn khi parse.
    assert!(serde_json::from_str::<Seat>(r#"{"id":"A1","row":"A","col":1,"status":"sold"}"#).is_err());
}
