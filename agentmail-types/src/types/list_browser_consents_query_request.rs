pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-browser-consents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBrowserConsentsQueryRequest {
    #[serde(default)]
    pub inbox_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<BrowserAuthorizationListLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
}

impl ListBrowserConsentsQueryRequest {
    pub fn builder() -> ListBrowserConsentsQueryRequestBuilder {
        <ListBrowserConsentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBrowserConsentsQueryRequestBuilder {
    inbox_id: Option<String>,
    limit: Option<BrowserAuthorizationListLimit>,
    page_token: Option<PageToken>,
}

impl ListBrowserConsentsQueryRequestBuilder {
    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: BrowserAuthorizationListLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBrowserConsentsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_id`](ListBrowserConsentsQueryRequestBuilder::inbox_id)
    pub fn build(self) -> Result<ListBrowserConsentsQueryRequest, BuildError> {
        Ok(ListBrowserConsentsQueryRequest {
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            limit: self.limit,
            page_token: self.page_token,
        })
    }
}

