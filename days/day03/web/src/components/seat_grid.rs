use leptos::{prelude::*, task::spawn_local};

use crate::{api::book_seat, models::Seat};

#[component]
pub fn SeatGrid(seats: Vec<Seat>) -> impl IntoView {
    // RwSignal ≈ useState, đọc/ghi qua .get()/.set(). Là Copy → move vào closure thoải mái.
    let error = RwSignal::new(None::<String>);

    view! {
        <div class="grid">
            {seats
                .into_iter()
                .map(|s| {
                    // Mỗi ghế 1 signal riêng → click ghế nào chỉ ghế đó render lại.
                    let booked = RwSignal::new(s.booked);
                    let title = format!("Hàng {} ghế {}", s.row, s.col);
                    let id = s.id.clone(); // closure cần SỞ HỮU 1 bản id riêng
                    let on_click = move |_| {
                        // Mỗi click clone lại, vì async block bên dưới lấy luôn bản này.
                        let id = id.clone();
                        // spawn_local ≈ gọi async không await trong onClick (fire-and-forget)
                        spawn_local(async move {
                            match book_seat(&id).await { // &id: cho MƯỢN, không move
                                Ok(_) => {
                                    booked.set(true);
                                    error.set(None);
                                }
                                Err(e) => error.set(Some(e)),
                            }
                        });
                    };
                    view! {
                        // class:booked=... ≈ className={booked ? 'seat booked' : 'seat'}
                        <button
                            class="seat"
                            class:booked=move || booked.get()
                            disabled=move || booked.get()
                            title=title
                            on:click=on_click
                        >
                            {s.id}
                        </button>
                    }
                })
                .collect_view()}
        </div>
        // Option<View>: None → không render gì (≈ {error && <p>...</p>})
        {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
    }
}
