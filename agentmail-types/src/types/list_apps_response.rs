pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAppsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub apps: Vec<App>,
}

impl ListAppsResponse {
    pub fn builder() -> ListAppsResponseBuilder {
        <ListAppsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAppsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    apps: Option<Vec<App>>,
}

impl ListAppsResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn apps(mut self, value: Vec<App>) -> Self {
        self.apps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAppsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListAppsResponseBuilder::count)
    /// - [`limit`](ListAppsResponseBuilder::limit)
    /// - [`apps`](ListAppsResponseBuilder::apps)
    pub fn build(self) -> Result<ListAppsResponse, BuildError> {
        Ok(ListAppsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            apps: self.apps.ok_or_else(|| BuildError::missing_field("apps"))?,
        })
    }
}
