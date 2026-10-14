use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub repo_url: String,
}
impl Url {
    pub fn new(repo_url: &str) -> Self {
        Self {
            repo_url: repo_url.into(),
        }
    }
}
