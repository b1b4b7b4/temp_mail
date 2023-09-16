use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::from_str;

#[derive(Deserialize, Debug, Clone)]
pub struct MailMessage {
    pub id: usize,
    pub from: String,
    pub subject: String,
    #[serde(deserialize_with = "from_str")]
    pub date: NaiveDateTime,
}
