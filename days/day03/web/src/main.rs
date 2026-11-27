// main.rs = entrypoint, chỉ mount App (≈ main.tsx: createRoot(...).render(<App/>)).
fn main() {
    leptos::mount::mount_to_body(web::app::App);
}
