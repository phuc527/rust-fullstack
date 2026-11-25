# Learn Rust — Seatly

Học Rust fullstack bằng cách xây **Seatly** (app đặt ghế rạp/sự kiện), mỗi ngày một bản nâng cấp chạy được.

- **Backend:** Axum
- **Frontend:** Leptos (Rust → WebAssembly)
- **Chạy:** Docker Compose (nginx phục vụ frontend + proxy `/api` sang backend)

Lộ trình chi tiết: [ROADMAP.md](ROADMAP.md)

## Cấu trúc
```
days/
  day01/   hello fullstack: /api/health
  day02/   lưới ghế: struct, Vec, serde
  ...
    api/   Axum backend
    web/   Leptos frontend
    docker-compose.yml
    README.md   ← đọc trước: khái niệm mới, so với Node, câu tự kiểm tra
```
Mỗi `dayXX` là bản đầy đủ, độc lập — có thể chạy bất kỳ ngày nào.

## Chạy
```sh
cd days/day02
docker compose up --build    # mở http://localhost:8080
docker compose down          # tắt trước khi chạy ngày khác (cùng cổng 8080)
```

## Chạy local (không Docker)
```sh
# backend
cd days/day02/api && cargo run

# frontend (cài 1 lần: rustup target add wasm32-unknown-unknown && cargo install trunk)
cd days/day02/web && trunk serve --proxy-backend=http://localhost:3000/api
```

## Yêu cầu
- Rust (rustup) — `rustc --version`
- Docker Desktop
- VS Code + rust-analyzer
