use super::command::Command;
use async_trait::async_trait;
use jira_api::Datasource;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[async_trait]
impl Command for Opt {
    async fn run(
        &self,
        http_client: &reqwest::Client,
        api_key: &str,
        username: &str,
        domain: &str,
    ) -> Result<(), anyhow::Error> {
        let datasource = jira_api::JiraApi::new(http_client, api_key, username, domain);
        let response = datasource.get_announcement_banner().await?;

        println!("{:#?}", response);

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Get,
}
