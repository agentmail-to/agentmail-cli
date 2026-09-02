pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchProvidersResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(default)]
    pub providers: Vec<Provider>,
}

impl SearchProvidersResponse {
    pub fn builder() -> SearchProvidersResponseBuilder {
        <SearchProvidersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchProvidersResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    providers: Option<Vec<Provider>>,
}

impl SearchProvidersResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn providers(mut self, value: Vec<Provider>) -> Self {
        self.providers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchProvidersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](SearchProvidersResponseBuilder::count)
    /// - [`limit`](SearchProvidersResponseBuilder::limit)
    /// - [`providers`](SearchProvidersResponseBuilder::providers)
    pub fn build(self) -> Result<SearchProvidersResponse, BuildError> {
        Ok(SearchProvidersResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            providers: self.providers.ok_or_else(|| BuildError::missing_field("providers"))?,
        })
    }
}
