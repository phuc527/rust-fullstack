// Config đọc từ env (≈ process.env.PORT ?? 3000). Production không hardcode cổng.
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok() // Result → Option (bỏ lỗi "không có biến")
            .and_then(|p| p.parse().ok()) // "3000" → u16, parse lỗi → None
            .unwrap_or(3000); // None → mặc định
        Self { port }
    }

    pub fn addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
