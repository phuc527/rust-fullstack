// Frontend viết bằng Rust → compile ra WebAssembly (.wasm), chạy trong browser.
// Leptos ≈ React/Solid; gloo-net ≈ fetch.
use gloo_net::http::Request;
use leptos::prelude::*;

// ≈ ReactDOM.createRoot(document.body).render(<App />)
fn main() {
    leptos::mount::mount_to_body(App);
}

// #[component] ≈ function component trong React.
// `impl IntoView` = "trả về thứ gì đó render được" (≈ JSX.Element).
#[component]
fn App() -> impl IntoView {
    // LocalResource ≈ useEffect + useState cho dữ liệu async (giống React Query thu nhỏ).
    // Chạy closure async 1 lần, lưu kết quả vào signal.
    let health = LocalResource::new(|| async {
        // `match` ≈ switch nhưng bắt buộc xử lý MỌI trường hợp (Ok và Err).
        match Request::get("/api/health").send().await {
            Ok(res) => res.text().await.unwrap_or_default(),
            Err(_) => "down".to_string(),
        }
    });

    // view! ≈ JSX. Text phải để trong "ngoặc kép".
    // `move || ...` = closure reactive: khi health có dữ liệu, chỉ đoạn này render lại
    // (không re-render cả component như React).
    // health.get() trả Option: None = đang tải, Some(text) = có kết quả.
    view! {
        <h1>"Seatly"</h1>
        <p>"API: " {move || health.get().unwrap_or_else(|| "loading...".to_string())}</p>
    }
}
