use serde::Serialize;

// struct ≈ TS interface + class data.
// #[derive(...)] = để compiler tự sinh code:
//   Serialize → struct → JSON (≈ JSON.stringify, sinh lúc compile, không reflection)
//   Clone     → copy sâu bằng .clone()
#[derive(Serialize, Clone, Debug)]
pub struct Seat {
    pub id: String, // "A1" — String = chuỗi có sở hữu, nằm trên heap
    pub row: char,  // 'A' — 1 ký tự Unicode, nháy đơn
    pub col: u8,    // 1..=10 — số nguyên không dấu 0..255. JS chỉ có `number`
}
// `pub` trên field: mặc định field là PRIVATE trong module. Khác TS (mặc định public).

// impl = gắn hàm vào type (≈ static method của class).
impl Seat {
    /// Lưới ghế cố định 5 hàng × 10 cột (Day 8 sẽ lấy từ DB).
    // Iterator chain ≈ ['A'..'E'].flatMap(row => [1..10].map(col => ...))
    // - 'A'..='E' : range ký tự, `..=` bao gồm E
    // - `move`    : closure trong lấy BẢN COPY của `row`
    // - collect() : iterator LAZY — không chạy gì cho tới khi collect thành Vec
    pub fn grid() -> Vec<Seat> {
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
}

// Unit test đặt cạnh code. #[cfg(test)] = chỉ compile khi `cargo test`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_is_5x10() {
        let seats = Seat::grid();
        assert_eq!(seats.len(), 50);
        assert_eq!(seats[0].id, "A1");
        assert_eq!(seats[49].id, "E10");
    }
}
