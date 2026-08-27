pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBrowserCredentialsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: BrowserAuthorizationListLimit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub credentials: Vec<BrowserCredential>,
}

impl ListBrowserCredentialsResponse {
    pub fn builder() -> ListBrowserCredentialsResponseBuilder {
        <ListBrowserCredentialsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBrowserCredentialsResponseBuilder {
    count: Option<Count>,
    limit: Option<BrowserAuthorizationListLimit>,
    next_page_token: Option<PageToken>,
    credentials: Option<Vec<BrowserCredential>>,
}

impl ListBrowserCredentialsResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: BrowserAuthorizationListLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn credentials(mut self, value: Vec<BrowserCredential>) -> Self {
        self.credentials = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBrowserCredentialsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListBrowserCredentialsResponseBuilder::count)
    /// - [`limit`](ListBrowserCredentialsResponseBuilder::limit)
    /// - [`credentials`](ListBrowserCredentialsResponseBuilder::credentials)
    pub fn build(self) -> Result<ListBrowserCredentialsResponse, BuildError> {
        Ok(ListBrowserCredentialsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            credentials: self.credentials.ok_or_else(|| BuildError::missing_field("credentials"))?,
        })
    }
}
