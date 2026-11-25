# Day 1 — Seatly fullstack "hello"

**Kết quả:** mở http://localhost:8080 → trang Leptos hiện `API: ok` (lấy từ Axum).

```sh
cd days/day01
docker compose up --build      # lần đầu ~5-10 phút (tải toolchain + compile)
```

## Kiến trúc
```
browser ──► web (nginx :8080)
              ├─ /        → file tĩnh: index.html + web.js + web_bg.wasm (Leptos)
              └─ /api/*   → proxy ──► api (Axum :3000)
```
Cùng 1 origin (8080) nên **không cần CORS**.

## Đọc theo thứ tự
| File | Học gì | Node tương đương |
|---|---|---|
| `api/Cargo.toml` | dependency, edition | `package.json` |
| `api/src/main.rs` | async main, Router, handler | Express hello |
| `web/Cargo.toml` | feature `csr` (client-side render) | — |
| `web/index.html` | `data-trunk` = Trunk tự chèn wasm vào | `index.html` của Vite |
| `web/src/main.rs` | component, signal, resource, `view!` | React + fetch |
| `web/nginx.conf` | serve static + reverse proxy | Vite proxy dev |
| `*/Dockerfile` | multi-stage: build bằng image rust, chạy bằng image nhỏ | `node:alpine` |

## Khái niệm chốt hôm nay
- **cargo** = npm + tsc + jest gộp lại. `cargo run`, `cargo build --release`, `cargo test`.
- **Compile ra binary**: image api chỉ chứa 1 file thực thi, không cần runtime (Node cần cả `node` + `node_modules`).
- **`.unwrap()`**: lấy giá trị, lỗi thì panic. Tạm dùng; Day 5 thay bằng error handling thật.
- **Option**: `Some(x)` / `None` thay cho `null/undefined` — compiler bắt bạn xử lý cả hai.
- **WASM**: Rust → `.wasm` chạy trong browser, Trunk ≈ Vite (build + bundle).

## Tự kiểm tra
1. Đổi `"ok"` thành `"healthy"` → rebuild → UI đổi?
2. `docker images` — so size `day01-api` với `node:22-alpine`.
3. Sửa `0.0.0.0` thành `127.0.0.1` → UI hiện gì? Vì sao?
