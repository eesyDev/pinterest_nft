use chrono::{DateTime, Utc};
pub struct Pin {
    pub id: String,
    pub user_id: String,
    pub nft_id: String,
    pub board_id: Option<String>,
    pub created_at: DateTime<Utc>
}