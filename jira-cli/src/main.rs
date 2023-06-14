use anyhow::Error;
use std::env;
use std::time;
use structopt::StructOpt;

mod presentation;
use presentation::command::Command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, env = "JIRA_API_KEY", hide_env_values = true)]
    api_key: String,

    #[structopt(long, env = "JIRA_USERNAME", hide_env_values = true)]
    username: String,

    #[structopt(long, env = "JIRA_DOMAIN", hide_env_values = true)]
    domain: String,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    AnnouncementBanner(presentation::announcement_banner::Opt),
    ApplicationRoles(presentation::application_roles::Opt),
    IssuePicker(presentation::issue_search::Opt),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Program started!");

    let opt = Opt::from_args();

    let http_client = reqwest::ClientBuilder::new()
        .timeout(time::Duration::from_secs(3))
        .build()?;

    let api_key = &opt.api_key;
    let username = &opt.username;
    let domain = &opt.domain;

    match opt.subcommand {
        Subcommand::AnnouncementBanner(opt) => {
            opt.run(&http_client, api_key, username, domain).await?
        }
        Subcommand::ApplicationRoles(opt) => {
            opt.run(&http_client, api_key, username, domain).await?
        }
        Subcommand::IssuePicker(opt) => opt.run(&http_client, api_key, username, domain).await?,
    };

    Ok(())
}
