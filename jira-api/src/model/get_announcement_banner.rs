use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub message: String,
    pub is_dismissible: bool,
    pub is_enabled: bool,
    pub hash_id: String,
    pub visibility: String,
}
