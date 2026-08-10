#[derive(Debug, Clone, serde::Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
