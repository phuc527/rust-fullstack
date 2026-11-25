// models/mod.rs ≈ models/index.ts: gom và re-export các model.
mod seat;

pub use seat::Seat; // cho phép `use crate::models::Seat` thay vì `models::seat::Seat`
