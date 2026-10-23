pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchAppsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(default)]
    pub apps: Vec<App>,
}

impl SearchAppsResponse {
    pub fn builder() -> SearchAppsResponseBuilder {
        <SearchAppsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchAppsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    apps: Option<Vec<App>>,
}

impl SearchAppsResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn apps(mut self, value: Vec<App>) -> Self {
        self.apps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchAppsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchAppsResponseBuilder::count)
    /// - [`limit`](SearchAppsResponseBuilder::limit)
    /// - [`apps`](SearchAppsResponseBuilder::apps)
    pub fn build(self) -> Result<SearchAppsResponse, BuildError> {
        Ok(SearchAppsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            apps: self.apps.ok_or_else(|| BuildError::missing_field("apps"))?,
        })
    }
}
