use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{attachment::Attachment, from_str};

#[derive(Deserialize, Debug)]
pub struct Message {
    pub id: usize,
    pub from: String,
    pub subject: String,
    #[serde(deserialize_with = "from_str")]
    pub date: NaiveDateTime,
    pub attachments: Vec<Attachment>,
    pub body: String,
    #[serde(alias = "textBody")]
    pub text_body: String,
    #[serde(alias = "htmlBody")]
    pub html_body: String,
}
