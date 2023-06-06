use async_trait::async_trait;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {}

pub struct TemplateApi<'a> {
    http_client: &'a reqwest::Client,
    base_url: String,
}

impl<'a> TemplateApi<'a> {
    pub fn new(http_client: &'a reqwest::Client) -> Self {
        Self {
            http_client,
            base_url: String::from("change me"),
        }
    }
}

#[async_trait]
impl<'a> Datasource for TemplateApi<'a> {}
