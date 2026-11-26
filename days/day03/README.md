# Day 3 — Đặt ghế: ownership, borrow, `&mut`, shared state

**Kết quả:** click 1 ghế → `POST /api/seats/{id}/book` → ghế chuyển đỏ. Click lại ghế đã đặt (từ tab khác) → hiện "Ghế đã có người đặt".

```sh
cd days/day03
docker compose up --build
```

## Cấu trúc (thêm so với Day 2)
```
api/src/
  state.rs         AppState = Arc<Mutex<Vec<Seat>>>       (≈ biến global seats trong Express)
  models/seat.rs   + field booked, + Seat::book(&mut self), + test
  routes/mod.rs    router(state) + .with_state(state)
  routes/seats.rs  list đọc state, + book (POST)
  main.rs          tạo state 1 lần → truyền vào router
web/src/
  api.rs           + book_seat(&str)
  models.rs        + booked
  components/seat_grid.rs   signal mỗi ghế, on:click, hiện lỗi
```

## Thay đổi so với Day 2
| File | Mới |
|---|---|
| `api/src/state.rs` | file mới: state dùng chung |
| `api/src/routes/seats.rs` | `book` trả `Result<Json<Seat>, StatusCode>` → 200 / 404 / 409 |
| `api/src/models/seat.rs` | `booked: bool`, `book(&mut self)`, test `book_twice_fails` |
| `web/src/components/seat_grid.rs` | `RwSignal`, `spawn_local`, `class:booked` |
| `web/index.html` | CSS `.seat.booked` |

## Khái niệm chốt hôm nay
- **Ownership**: mỗi giá trị có đúng 1 chủ. Gán/truyền `String` = **move** (chủ cũ không dùng được nữa). Muốn giữ cả hai → `.clone()`.
- **Borrow `&T`**: cho mượn để đọc, không lấy quyền sở hữu. `book_seat(&id)` → `id` vẫn dùng được sau đó.
- **Borrow `&mut T`**: mượn để sửa. Tại 1 thời điểm chỉ có **1** `&mut` *hoặc* nhiều `&` — đó là cách compiler chặn data race.
- **`mut`**: biến mặc định bất biến. `let mut seats = ...` mới sửa được.
- **`Arc<Mutex<T>>`**: `Arc` = chia sẻ giữa nhiều thread, `Mutex` = khóa khi sửa. Guard tự mở khóa khi ra khỏi scope (RAII — không có `unlock()` để quên).
- **Extractor**: `State(state)`, `Path(id)` là tham số handler, Axum tự điền (≈ `req.app.locals`, `req.params`).
- **`Option::ok_or(...)?`**: `None` → trả lỗi sớm. Gọn hơn `if (!x) return ...`.
- **Signal**: `RwSignal::new(v)` ≈ `useState(v)`. `.get()` trong closure `move ||` → tự cập nhật DOM, không re-render cả component như React.
- **`spawn_local`**: chạy async trong event handler (≈ gọi async fn trong `onClick` không await).

## So với Node
```js
// Node/Express — 1 thread nên sửa thẳng biến global
app.post('/api/seats/:id/book', (req, res) => {
  const seat = seats.find(s => s.id === req.params.id);
  if (!seat) return res.sendStatus(404);
  if (seat.booked) return res.sendStatus(409);
  seat.booked = true;
  res.json(seat);
});
```
Rust chạy đa luồng: nếu quên `Mutex`, **không compile**. Node không có vấn đề này lúc này, nhưng gặp lại khi scale nhiều instance (Day 9 giải bằng transaction DB).

## Tự kiểm tra
1. `cd api && cargo test` — 2 test pass. Bỏ `mut` ở `let mut seat` trong test → lỗi gì?
2. Trong `seat_grid.rs`, bỏ dòng `let id = id.clone();` trong `on_click` → compiler nói gì? (gợi ý: closure `FnMut` gọi nhiều lần nhưng `async move` lấy mất `id`)
3. Đổi `book_seat(id: &str)` thành `book_seat(id: String)` — chỗ gọi phải sửa thế nào?
4. Mở 2 tab, đặt A5 ở tab 1, rồi click A5 ở tab 2 → thấy gì? Vì sao tab 2 chưa đỏ sẵn? (Day 15 làm realtime)
5. `docker compose restart api` → reload: ghế đã đặt còn không? Vì sao?
6. `curl -X POST localhost:8080/api/seats/Z9/book -i` → status gì?
