pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBrowserConsentsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: BrowserAuthorizationListLimit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub consents: Vec<BrowserConsent>,
}

impl ListBrowserConsentsResponse {
    pub fn builder() -> ListBrowserConsentsResponseBuilder {
        <ListBrowserConsentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBrowserConsentsResponseBuilder {
    count: Option<Count>,
    limit: Option<BrowserAuthorizationListLimit>,
    next_page_token: Option<PageToken>,
    consents: Option<Vec<BrowserConsent>>,
}

impl ListBrowserConsentsResponseBuilder {
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

    pub fn consents(mut self, value: Vec<BrowserConsent>) -> Self {
        self.consents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBrowserConsentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListBrowserConsentsResponseBuilder::count)
    /// - [`limit`](ListBrowserConsentsResponseBuilder::limit)
    /// - [`consents`](ListBrowserConsentsResponseBuilder::consents)
    pub fn build(self) -> Result<ListBrowserConsentsResponse, BuildError> {
        Ok(ListBrowserConsentsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            consents: self.consents.ok_or_else(|| BuildError::missing_field("consents"))?,
        })
    }
}
