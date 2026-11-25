use leptos::prelude::*;

use crate::{api::fetch_seats, components::SeatGrid};

#[component]
pub fn App() -> impl IntoView {
    let seats = LocalResource::new(fetch_seats);

    view! {
        <h1>"Seatly"</h1>
        <div class="screen">"MÀN HÌNH"</div>
        // seats.get(): None = đang tải | Some(Ok) = data | Some(Err) = lỗi
        // Mỗi nhánh khác kiểu view → .into_any() gom về 1 kiểu (≈ ReactNode)
        {move || match seats.get() {
            None => view! { <p>"Đang tải..."</p> }.into_any(),
            Some(Err(e)) => view! { <p class="error">"Lỗi: " {e}</p> }.into_any(),
            Some(Ok(list)) => view! { <SeatGrid seats=list /> }.into_any(),
        }}
    }
}
