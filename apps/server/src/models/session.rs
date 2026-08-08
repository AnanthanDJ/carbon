#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub token: String,
    pub user_id: String,
    pub created_at: String,
}
