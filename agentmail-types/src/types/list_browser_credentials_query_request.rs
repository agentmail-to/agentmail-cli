pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-browser-credentials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBrowserCredentialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<BrowserAuthorizationListLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl ListBrowserCredentialsQueryRequest {
    pub fn builder() -> ListBrowserCredentialsQueryRequestBuilder {
        <ListBrowserCredentialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBrowserCredentialsQueryRequestBuilder {
    limit: Option<BrowserAuthorizationListLimit>,
    page_token: Option<PageToken>,
}

impl ListBrowserCredentialsQueryRequestBuilder {
    pub fn limit(mut self, value: BrowserAuthorizationListLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBrowserCredentialsQueryRequest`].
    pub fn build(self) -> Result<ListBrowserCredentialsQueryRequest, BuildError> {
        Ok(ListBrowserCredentialsQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

