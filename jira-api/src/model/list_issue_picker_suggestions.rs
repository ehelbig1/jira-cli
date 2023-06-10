use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize)]
pub struct Section {
    pub id: String,
    pub issues: Vec<Issue>,
    pub label: String,
    pub msg: Option<String>,
    pub sub: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: usize,
    pub img: String,
    pub key: String,
    pub key_html: String,
    pub summary: String,
    pub summary_text: String,
}
