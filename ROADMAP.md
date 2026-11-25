# Seatly — học Rust fullstack, mỗi ngày 1 bản code chạy được

Mỗi ngày = 1 thư mục `days/dayXX/` **đầy đủ code** (api Axum + web Leptos + Docker), tăng dần độ khó.
Cách học: đọc `README.md` của ngày → đọc code (có comment so với Node) → `docker compose up --build` → trả lời "Tự kiểm tra".

Stack: **Axum** · **Leptos (WASM)** · **Postgres + sqlx** (từ Day 8) · **nginx** · **Docker Compose**

| Day | Backend (api) | Frontend (web) | Khái niệm Rust chính | Level |
|---|---|---|---|---|
| 1 ✅ | `GET /api/health` | Trang gọi API, hiện status | cargo, async, Option, WASM | ⭐ |
| 2 | `GET /api/seats` trả lưới 5×10 (`struct Seat` + serde) | Render lưới ghế | struct, Vec, iterator, serde, `For` | ⭐ |
| 3 | `POST /api/seats/{id}/book`, state `Arc<Mutex>` | Click ghế → đặt, đổi màu | ownership, borrow, `&mut`, signal set | ⭐⭐ |
| 4 | `enum SeatStatus {Free, Held, Booked}` | Màu theo status, chú thích | enum, `match`, exhaustive | ⭐⭐ |
| 5 | `AppError` → 404/409, `IntoResponse` | Hiện lỗi "ghế đã có người đặt" | `Result`, `?`, trait | ⭐⭐ |
| 6 | Workspace + crate `shared` (type dùng chung FE/BE) | Dùng `shared::Seat`, bỏ type trùng | modules, crates, workspace | ⭐⭐ |
| 7 | Unit test domain | Component tách file (`SeatGrid`, `Seat`) | test, props, `impl Trait` | ⭐⭐ |
| 8 | Postgres + sqlx + migration | Loading/error state đẹp hơn | async DB, pool, `Suspense` | ⭐⭐⭐ |
| 9 | Transaction `FOR UPDATE` chống đặt trùng | Optimistic update + rollback khi 409 | transaction, concurrency | ⭐⭐⭐ |
| 10 | Nhiều event (suất chiếu), quan hệ 1-n | Router: danh sách event → sơ đồ ghế | `leptos_router`, params | ⭐⭐⭐ |
| 11 | Chọn nhiều ghế, 1 order nhiều ghế | Giỏ ghế, tổng tiền | collections, `HashSet`, lifetimes | ⭐⭐⭐ |
| 12 | Register/login: argon2 + JWT, extractor `AuthUser` | Form login, lưu token, guard route | custom extractor, trait impl | ⭐⭐⭐⭐ |
| 13 | "Vé của tôi" | Trang vé của tôi | generics, `FromRow` | ⭐⭐⭐ |
| 14 | Hold ghế 5 phút + job nhả ghế (`tokio::spawn`) | Đồng hồ đếm ngược | tasks, interval, `Send` | ⭐⭐⭐⭐ |
| 15 | WebSocket + `broadcast` channel | Ghế realtime giữa các tab | channels, `Arc`, streams | ⭐⭐⭐⭐ |
| 16 | Validation, rate limit (`tower`), `tracing` | Form validation | middleware, Layer | ⭐⭐⭐⭐ |
| 17 | Integration test (`sqlx::test`) + load test (`oha`) vs bản Node | — | test async, benchmark | ⭐⭐⭐⭐ |
| 18 | Admin: tạo event, phân quyền | Trang admin | enum role, trait bound | ⭐⭐⭐⭐ |
| 19 | `cargo-chef`, distroless, image < 30MB | — | build tối ưu | ⭐⭐⭐ |
| 20 | **SSR**: Axum + Leptos chung 1 binary (`cargo-leptos`) | Server functions, hydrate | SSR, `#[server]` | ⭐⭐⭐⭐⭐ |

Mỗi ngày nhắn **"day X"** → tôi tạo `days/dayXX/` dựa trên day trước + README giải thích phần mới.
