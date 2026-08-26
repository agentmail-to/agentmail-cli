pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListApiKeysResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `created_at` descending.
    #[serde(default)]
    pub api_keys: Vec<ApiKey>,
}

impl ListApiKeysResponse {
    pub fn builder() -> ListApiKeysResponseBuilder {
        <ListApiKeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApiKeysResponseBuilder {
    count: Option<Count>,
    next_page_token: Option<PageToken>,
    api_keys: Option<Vec<ApiKey>>,
}

impl ListApiKeysResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn api_keys(mut self, value: Vec<ApiKey>) -> Self {
        self.api_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListApiKeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListApiKeysResponseBuilder::count)
    /// - [`api_keys`](ListApiKeysResponseBuilder::api_keys)
    pub fn build(self) -> Result<ListApiKeysResponse, BuildError> {
        Ok(ListApiKeysResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            next_page_token: self.next_page_token,
            api_keys: self.api_keys.ok_or_else(|| BuildError::missing_field("api_keys"))?,
        })
    }
}
