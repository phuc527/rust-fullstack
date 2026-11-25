# Day 2 — Danh sách ghế: struct, Vec, serde

**Kết quả:** http://localhost:8080 hiện sơ đồ 5 hàng × 10 ghế (A1…E10) lấy từ `GET /api/seats`.

```sh
cd days/day02
docker compose up --build
```

## Cấu trúc (chuẩn production, dùng từ Day 2 trở đi)
```
api/src/
  main.rs          entrypoint: đọc config, bind cổng, serve      (≈ server.ts)
  lib.rs           khai báo module → test/integration test import được
  config.rs        Config::from_env() — PORT từ env                (≈ process.env)
  models/
    mod.rs         re-export                                       (≈ index.ts)
    seat.rs        struct Seat + Seat::grid() + unit test          (domain)
  routes/
    mod.rs         router(): nest("/api", ...)                     (≈ app.use('/api', r))
    health.rs      GET /api/health
    seats.rs       GET /api/seats — handler mỏng ≈ controller
web/src/
  main.rs          mount App                                       (≈ main.tsx)
  app.rs           App: gọi API, xử lý loading/lỗi/data
  api.rs           fetch_seats() — lớp HTTP                        (≈ src/api/*.ts)
  models.rs        struct Seat (Deserialize)
  components/
    mod.rs
    seat_grid.rs   <SeatGrid seats=.../>
```
Quy tắc: `main` không chứa logic · handler/component mỏng · logic nằm ở `models` · HTTP client tách riêng.

## Thay đổi so với Day 1
| File | Mới |
|---|---|
| `api/Cargo.toml`, `web/Cargo.toml` | + `serde` |
| `api/src/*` | tách module như trên, `GET /api/seats`, unit test |
| `web/src/*` | `fetch_seats()` trả `Result`, component `SeatGrid` có props, 3 trạng thái |
| `web/index.html` | CSS lưới ghế |

## Khái niệm chốt hôm nay
- **struct**: gom dữ liệu có kiểu rõ ràng. `String` (chuỗi sở hữu), `char` (1 ký tự), `u8` (0–255).
- **derive**: `#[derive(Serialize, Deserialize, Clone)]` — compiler tự sinh code JSON. Sai field/kiểu → lỗi khi parse, không âm thầm `undefined`.
- **Vec<T>** ≈ Array có kiểu.
- **Iterator lazy**: `map`/`flat_map` không chạy cho tới `collect()`. `move` closure copy biến vào trong.
- **Result + `?`**: lỗi là một giá trị trong kiểu trả về; `?` = trả lỗi ra ngoài sớm.
- **match 3 nhánh**: loading (`None`) / lỗi (`Some(Err)`) / data (`Some(Ok)`). Bỏ 1 nhánh → compile lỗi.
- **Component props**: `fn SeatGrid(seats: Vec<Seat>)` ≈ `function SeatGrid({ seats })`.
- **Module**: `mod x;` khai báo file `x.rs`/`x/mod.rs`. Rust không tự nhận file — quên `mod` = file không được compile. `pub` mới dùng được từ ngoài; `pub use` = re-export; `crate::` = gốc crate.
- **lib + bin**: `main.rs` gọi `use api::routes` từ `lib.rs` — cùng 1 crate có 2 target.
- **`#[cfg(test)]`**: test sống cạnh code, `cargo test` để chạy.

## So với Node
```js
// Node/Express
app.get('/api/seats', (req, res) =>
  res.json(['A','B','C','D','E'].flatMap(row =>
    Array.from({ length: 10 }, (_, i) => ({ id: `${row}${i + 1}`, row, col: i + 1 })))));
```
Ngắn tương đương — nhưng bản Rust: struct định nghĩa shape 1 lần, FE parse sai shape sẽ báo lỗi rõ ràng.

## Tự kiểm tra
1. `cd api && cargo test` — sửa `50` thành `51`, đọc thông báo lỗi.
2. Giữ `col: u8` ở api, đổi thành `col: String` ở web. UI hiện gì? (gợi ý: nhánh `Err`)
3. Bỏ `move` trong `Seat::grid()` — compiler nói gì về `row`?
4. Xóa dòng `mod health;` trong `routes/mod.rs` — lỗi gì?
5. Tắt container api (`docker compose stop api`) rồi reload trang — thấy nhánh nào?
