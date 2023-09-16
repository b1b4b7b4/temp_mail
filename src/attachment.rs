use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Attachment {
    pub filename: String,
    #[serde(alias = "contentType")]
    pub content_type: String,
    pub size: usize,
}
