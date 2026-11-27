// models/mod.rs ≈ models/index.ts: gom và re-export các model.
mod seat;

pub use seat::{Seat, SeatStatus}; // cho phép `use crate::models::Seat` thay vì `models::seat::Seat`
