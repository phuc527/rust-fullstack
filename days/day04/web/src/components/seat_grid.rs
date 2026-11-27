use leptos::{prelude::*, task::spawn_local};

use crate::{
    api::{book_seat, cancel_seat},
    models::{Seat, SeatStatus},
};

#[component]
pub fn SeatGrid(seats: Vec<Seat>) -> impl IntoView {
    let error = RwSignal::new(None::<String>);

    view! {
        // Chú thích màu. Lặp qua mảng enum ≈ [...].map(s => <span>) trong React.
        <div class="legend">
            {[SeatStatus::Free, SeatStatus::Held, SeatStatus::Booked]
                .map(|s| view! { <span class=s.class()>{s.label()}</span> })
                .collect_view()}
        </div>
        <div class="grid">
            {seats
                .into_iter()
                .map(|s| {
                    // Signal giữ cả enum thay vì bool. SeatStatus là Copy → .get() trả bản copy.
                    let status = RwSignal::new(s.status);
                    let title = format!("Hàng {} ghế {}", s.row, s.col);
                    let id = s.id.clone();
                    let on_click = move |_| {
                        let id = id.clone();
                        // match bắt buộc đủ case: thêm trạng thái mới mà quên ở đây → lỗi compile.
                        let current = status.get_untracked();
                        if current == SeatStatus::Held {
                            error.set(Some("Ghế đang có người giữ".into()));
                            return;
                        }
                        spawn_local(async move {
                            let res = match current {
                                SeatStatus::Booked => cancel_seat(&id).await,
                                SeatStatus::Free | SeatStatus::Held => book_seat(&id).await,
                            };
                            match res {
                                Ok(seat) => {
                                    status.set(seat.status);
                                    error.set(None);
                                }
                                Err(e) => error.set(Some(e)),
                            }
                        });
                    };
                    view! {
                        // class động từ enum: "seat free" / "seat held" / "seat booked"
                        <button
                            class=move || format!("seat {}", status.get().class())
                            title=move || format!("{title} — {}", status.get().label())
                            on:click=on_click
                        >
                            {s.id.clone()}
                        </button>
                    }
                })
                .collect_view()}
        </div>
        {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
    }
}
