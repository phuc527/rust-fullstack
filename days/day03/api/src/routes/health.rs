// Handler = async fn. Kiểu trả về quyết định response (&str → text/plain).
pub async fn health() -> &'static str {
    "ok"
}
