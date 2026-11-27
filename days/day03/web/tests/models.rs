// Integration test (≈ __tests__/models.test.ts). Mỗi file trong tests/ là 1 crate riêng,
// chỉ dùng được API `pub` của lib — test như người ngoài dùng.
use web::models::Seat;

#[test]
fn parse_seat_json() {
    let s: Seat = serde_json::from_str(r#"{"id":"A1","row":"A","col":1,"booked":true}"#).unwrap();
    assert_eq!(s.id, "A1");
    assert!(s.booked);
}

#[test]
fn missing_field_is_error() {
    // JS sẽ cho `booked: undefined`; Rust báo lỗi ngay.
    assert!(serde_json::from_str::<Seat>(r#"{"id":"A1","row":"A","col":1}"#).is_err());
}
