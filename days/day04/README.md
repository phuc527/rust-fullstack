# Day 4 — Trạng thái ghế: `enum`, `match`, `Result`

**Kết quả:** ghế có 3 trạng thái `free` / `held` / `booked`, mỗi trạng thái 1 màu, có chú thích màu phía trên. C5, C6 đang `held` (giả lập người khác giữ) → click báo "Ghế đang có người giữ".

```sh
cd days/day04
docker compose up --build
```

## Thay đổi so với Day 3
| File | Mới |
|---|---|
| `api/src/models/seat.rs` | `enum SeatStatus`, `booked: bool` → `status`, `book()/cancel()` trả `Result<(), SeatStatus>` |
| `api/src/routes/seats.rs` | `if seat.book().is_err()` |
| `api/tests/seats_api.rs` | + `held_seat_is_409` |
| `web/src/models.rs` | enum + `class()` / `label()` |
| `web/src/components/seat_grid.rs` | signal giữ enum, legend, class động |
| `web/index.html` | CSS `.held`, `.legend` |

JSON giờ là `{"id":"A1","row":"A","col":1,"status":"free"}`.

## Khái niệm chốt hôm nay
- **`enum`** ≈ TS union `'free' | 'held' | 'booked'`. Khác ở chỗ: Rust chặn giá trị lạ ngay lúc parse JSON (`"sold"` → lỗi), TS thì chỉ check lúc compile.
- **`match` phải đủ case** (exhaustive). Thêm `SeatStatus::Sold` → mọi `match` thiếu case đều lỗi compile. `switch` của JS thì lặng lẽ rơi vào `default`.
- **Pattern**: `('C', 5 | 6)` match tuple, `|` = hoặc, `_` = còn lại, `other =>` bắt giá trị vào biến.
- **`Result<T, E>`** thay `bool`: lỗi mang thông tin (`Err(SeatStatus::Held)` cho biết *vì sao* không đặt được).
- **`#[serde(rename_all = "lowercase")]`**: `Free` ↔ `"free"`.
- **`Copy`** trên enum nhỏ: gán/truyền là copy, không move → `status.get()` thoải mái.
- **`&'static str`**: chuỗi nằm sẵn trong binary, không cấp phát (≈ string literal hằng).

## So với TS
```ts
type SeatStatus = 'free' | 'held' | 'booked';
function book(s: Seat) {
  switch (s.status) {
    case 'free': s.status = 'booked'; return true;
    default: return false; // thêm 'sold' → không ai báo
  }
}
```

## Tự kiểm tra
1. Thêm `Sold` vào enum bên api → `cargo build` báo lỗi ở đâu? Bên web thì sao?
2. Trong `book()`, đổi `other => Err(other)` thành `SeatStatus::Booked => Err(...)` → compiler nói gì?
3. Bỏ `Copy` trong derive của web `SeatStatus` → chỗ nào lỗi? Vì sao?
4. `curl -X POST localhost:8080/api/seats/C5/book -i` → status? Vì sao chưa phân biệt được "held" với "booked" (gợi ý: Day 5 làm error JSON).
5. `cd api && cargo test` — 3 unit + 3 integration pass.
