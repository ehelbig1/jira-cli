use async_trait::async_trait;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn get_announcement_banner(
        &self,
    ) -> Result<model::get_announcement_banner::Response, error::Error>;
    async fn list_application_roles(
        &self,
    ) -> Result<model::list_application_roles::Response, error::Error>;
    async fn list_issue_picker_suggestions(
        &self,
        query: &str,
    ) -> Result<model::list_issue_picker_suggestions::Response, error::Error>;
}

pub struct JiraApi<'a> {
    http_client: &'a reqwest::Client,
    api_key: String,
    base_url: String,
}

impl<'a> JiraApi<'a> {
    pub fn new(
        http_client: &'a reqwest::Client,
        api_key: &str,
        username: &str,
        domain: &str,
    ) -> Self {
        let api_key = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!("{}:{}", username, api_key),
        );

        Self {
            http_client,
            api_key,
            base_url: String::from(domain),
        }
    }
}

#[async_trait]
impl<'a> Datasource for JiraApi<'a> {
    async fn get_announcement_banner(
        &self,
    ) -> Result<model::get_announcement_banner::Response, error::Error> {
        let url = format!("{}/rest/api/3/announcementBanner", self.base_url);

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    async fn list_application_roles(
        &self,
    ) -> Result<model::list_application_roles::Response, error::Error> {
        let url = format!("{}/rest/api/3/applicationrole", self.base_url);

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    async fn list_issue_picker_suggestions(
        &self,
        query: &str,
    ) -> Result<model::list_issue_picker_suggestions::Response, error::Error> {
        let url = format!("{}/rest/api/3/issue/picker", self.base_url);

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .query(&[("query", query)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}
