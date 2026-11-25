use axum::{Json, Router, routing::get};
use serde::Serialize;

// NEW Day 2: struct ≈ TS interface + class data.
// #[derive(...)] = để compiler tự sinh code:
//   Serialize → biến struct thành JSON (≈ JSON.stringify, nhưng sinh lúc compile, không dùng reflection)
//   Clone     → cho phép copy sâu bằng .clone()
#[derive(Serialize, Clone)]
struct Seat {
    id: String, // "A1" — String = chuỗi có sở hữu, nằm trên heap (≈ string của JS)
    row: char,  // 'A' — char = 1 ký tự Unicode, dùng nháy đơn
    col: u8,    // 1..=10 — u8 = số nguyên không dấu 0..255. JS chỉ có `number`
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/seats", get(seats)); // NEW

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("api on :3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

// Json(...) là wrapper: Axum serialize + set header content-type: application/json
// ≈ res.json(allSeats())
async fn seats() -> Json<Vec<Seat>> {
    Json(all_seats())
}

// Iterator chain ≈ ['A'..'E'].flatMap(row => [1..10].map(col => ...))
// - 'A'..='E'  : range ký tự, `..=` là bao gồm cả E
// - flat_map   : như flatMap của JS
// - `move`     : closure trong lấy BẢN COPY của `row` (vì nó chạy sau khi closure ngoài đã xong vòng)
// - format!    : ≈ template string `${row}${col}`
// - collect()  : iterator trong Rust là LAZY — không chạy gì cho tới khi collect thành Vec
fn all_seats() -> Vec<Seat> {
    ('A'..='E')
        .flat_map(|row| {
            (1..=10).map(move |col| Seat {
                id: format!("{row}{col}"),
                row,
                col,
            })
        })
        .collect()
}

// `cargo test` chạy hàm này. #[cfg(test)] = chỉ compile khi test, không vào binary release.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_is_5x10() {
        let seats = all_seats();
        assert_eq!(seats.len(), 50);
        assert_eq!(seats[0].id, "A1");
        assert_eq!(seats[49].id, "E10");
    }
}
