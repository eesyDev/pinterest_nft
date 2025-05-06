use chrono::{DateTime, Utc};
use super::pin::Pin;
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub wallet_address: String,
    pub profile_picture_url: Option<String>,
    pub bio: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub social_links: Vec<Option<String>>,
    pub pins: Vec<Pin>
}

