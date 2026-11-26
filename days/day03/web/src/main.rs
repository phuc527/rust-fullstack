// main.rs = entrypoint, chỉ mount App (≈ main.tsx: createRoot(...).render(<App/>)).
mod api;
mod app;
mod components;
mod models;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
