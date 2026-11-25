use leptos::prelude::*;

use crate::models::Seat;

// Component có props = tham số hàm (≈ function SeatGrid({ seats })).
#[component]
pub fn SeatGrid(seats: Vec<Seat>) -> impl IntoView {
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
