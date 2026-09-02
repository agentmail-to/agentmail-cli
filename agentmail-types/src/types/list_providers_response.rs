pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProvidersResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub providers: Vec<Provider>,
}

impl ListProvidersResponse {
    pub fn builder() -> ListProvidersResponseBuilder {
        <ListProvidersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProvidersResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    providers: Option<Vec<Provider>>,
}

impl ListProvidersResponseBuilder {
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

    pub fn providers(mut self, value: Vec<Provider>) -> Self {
        self.providers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProvidersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListProvidersResponseBuilder::count)
    /// - [`limit`](ListProvidersResponseBuilder::limit)
    /// - [`providers`](ListProvidersResponseBuilder::providers)
    pub fn build(self) -> Result<ListProvidersResponse, BuildError> {
        Ok(ListProvidersResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            providers: self.providers.ok_or_else(|| BuildError::missing_field("providers"))?,
        })
    }
}
