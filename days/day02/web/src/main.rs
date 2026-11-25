use gloo_net::http::Request;
use leptos::prelude::*;
use serde::Deserialize;

// NEW Day 2: struct giống hệt bên api, nhưng Deserialize (JSON → struct).
// Đang bị trùng code với api — Day 6 sẽ gom vào crate `shared` dùng chung.
// Nếu JSON thiếu field hoặc sai kiểu → lỗi ngay khi parse (JS thì nhận `undefined` âm thầm).
#[derive(Deserialize, Clone)]
struct Seat {
    id: String,
    row: char,
    col: u8,
}

fn main() {
    leptos::mount::mount_to_body(App);
}

// Gọi API, trả Result<Vec<Seat>, String>.
// `?` = nếu Err thì return Err ngay (≈ throw nhưng hiện rõ trong kiểu trả về).
// map_err: đổi lỗi gloo_net thành String cho dễ hiển thị (và để Clone được).
async fn fetch_seats() -> Result<Vec<Seat>, String> {
    let res = Request::get("/api/seats")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    res.json::<Vec<Seat>>().await.map_err(|e| e.to_string())
}

#[component]
fn App() -> impl IntoView {
    let seats = LocalResource::new(fetch_seats);

    view! {
        <h1>"Seatly"</h1>
        <div class="screen">"MÀN HÌNH"</div>
        // seats.get(): None = đang tải | Some(Ok) = có data | Some(Err) = lỗi
        // Mỗi nhánh trả về view khác kiểu → .into_any() để gom về 1 kiểu (≈ ReactNode)
        {move || match seats.get() {
            None => view! { <p>"Đang tải..."</p> }.into_any(),
            Some(Err(e)) => view! { <p class="error">"Lỗi: " {e}</p> }.into_any(),
            Some(Ok(list)) => view! { <SeatGrid seats=list /> }.into_any(),
        }}
    }
}

// NEW: component có props. Props = tham số hàm (≈ function SeatGrid({ seats })).
#[component]
fn SeatGrid(seats: Vec<Seat>) -> impl IntoView {
    view! {
        <div class="grid">
            // into_iter().map(...).collect_view() ≈ seats.map(s => <button/>) trong JSX
            {seats
                .into_iter()
                .map(|s| {
                    let title = format!("Hàng {} ghế {}", s.row, s.col);
                    view! { <button class="seat" title=title>{s.id}</button> }
                })
                .collect_view()}
        </div>
    }
}
