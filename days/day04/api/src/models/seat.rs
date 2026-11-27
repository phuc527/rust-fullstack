use serde::Serialize;

// enum ≈ TS union `type SeatStatus = 'free' | 'held' | 'booked'`, nhưng compiler bắt
// mọi `match` phải xử lý ĐỦ các trường hợp → thêm trạng thái mới là chỗ nào sót đều lỗi compile.
// rename_all: Free → "free" trong JSON (≈ convention của JS).
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeatStatus {
    Free,
    Held, // đang có người giữ chờ thanh toán (Day 10 có timeout thật)
    Booked,
}

#[derive(Serialize, Clone, Debug)]
pub struct Seat {
    pub id: String,
    pub row: char,
    pub col: u8,
    pub status: SeatStatus, // thay cho `booked: bool` — bool chỉ có 2 trạng thái, không đủ
}

impl Seat {
    /// Lưới 5×10. Vài ghế hàng C đang Held để thấy màu (giả lập người khác đang giữ).
    pub fn grid() -> Vec<Seat> {
        ('A'..='E')
            .flat_map(|row| {
                (1..=10).map(move |col| Seat {
                    id: format!("{row}{col}"),
                    row,
                    col,
                    // match trên tuple ≈ switch nhiều điều kiện. `_` = mọi trường hợp còn lại.
                    status: match (row, col) {
                        ('C', 5 | 6) => SeatStatus::Held,
                        _ => SeatStatus::Free,
                    },
                })
            })
            .collect()
    }

    /// Đặt ghế: chỉ Free → Booked. Err trả trạng thái hiện tại để caller báo lỗi đúng.
    // Result<(), SeatStatus> ≈ hàm trả `void` hoặc throw, nhưng lỗi nằm trong kiểu trả về.
    pub fn book(&mut self) -> Result<(), SeatStatus> {
        match self.status {
            SeatStatus::Free => {
                self.status = SeatStatus::Booked;
                Ok(())
            }
            other => Err(other), // Held hoặc Booked
        }
    }

    /// Gỡ đặt: chỉ Booked → Free.
    pub fn cancel(&mut self) -> Result<(), SeatStatus> {
        match self.status {
            SeatStatus::Booked => {
                self.status = SeatStatus::Free;
                Ok(())
            }
            other => Err(other),
        }
    }
}

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

    #[test]
    fn book_and_cancel() {
        let mut seat = Seat::grid().remove(0);
        assert_eq!(seat.book(), Ok(()));
        assert_eq!(seat.book(), Err(SeatStatus::Booked));
        assert_eq!(seat.cancel(), Ok(()));
        assert_eq!(seat.cancel(), Err(SeatStatus::Free));
    }

    #[test]
    fn held_seat_cannot_be_booked() {
        let mut seat = Seat::grid().into_iter().find(|s| s.id == "C5").unwrap();
        assert_eq!(seat.book(), Err(SeatStatus::Held));
    }
}
