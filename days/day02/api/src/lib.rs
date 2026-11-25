// lib.rs = gốc của "thư viện" api. Crate có cả main.rs + lib.rs:
//   - lib  : chứa toàn bộ code (models, routes...) → test/integration test import được
//   - bin  : main.rs chỉ gọi vào lib (`use api::...`)
// `pub mod x;` ≈ khai báo file x.rs (hoặc x/mod.rs) là 1 module public.
// Khác Node: Rust KHÔNG tự nhận file — file nào không được `mod` thì không compile.
pub mod config;
pub mod models;
pub mod routes;
