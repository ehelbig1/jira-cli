use serde::Deserialize;

pub type Response = Vec<ApplicationRole>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationRole {
    pub key: String,
    pub groups: Vec<String>,
    pub group_details: Vec<GroupDetail>,
    pub name: String,
    pub default_groups: Vec<String>,
    pub default_group_details: Vec<GroupDetail>,
    pub selected_by_default: bool,
    pub defined: bool,
    pub number_of_seats: usize,
    pub remaining_seats: usize,
    pub user_count: usize,
    pub user_count_description: String,
    pub has_unlimited_seats: bool,
    pub platform: bool,
}

#[derive(Debug, Deserialize)]
pub struct GroupDetail {
    pub name: String,
    pub group_id: String,

    #[serde(rename = "self")]
    pub own: url::Url,
}
