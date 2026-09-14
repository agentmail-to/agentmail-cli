pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysListQueryRequest {
    /// Restrict the list to one credential family. Omit for every family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ApiKeyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<Ascending>,
}

impl ApiKeysListQueryRequest {
    pub fn builder() -> ApiKeysListQueryRequestBuilder {
        <ApiKeysListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysListQueryRequestBuilder {
    r#type: Option<ApiKeyType>,
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    ascending: Option<Ascending>,
}

impl ApiKeysListQueryRequestBuilder {
    pub fn r#type(mut self, value: ApiKeyType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    pub fn ascending(mut self, value: Ascending) -> Self {
        self.ascending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysListQueryRequest`].
    pub fn build(self) -> Result<ApiKeysListQueryRequest, BuildError> {
        Ok(ApiKeysListQueryRequest {
            r#type: self.r#type,
            limit: self.limit,
            page_token: self.page_token,
            ascending: self.ascending,
        })
    }
}

