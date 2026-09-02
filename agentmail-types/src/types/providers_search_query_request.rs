pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProvidersSearchQueryRequest {
    /// Name prefix to search for.
    #[serde(default)]
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
}

impl ProvidersSearchQueryRequest {
    pub fn builder() -> ProvidersSearchQueryRequestBuilder {
        <ProvidersSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProvidersSearchQueryRequestBuilder {
    q: Option<String>,
    limit: Option<Limit>,
}

impl ProvidersSearchQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProvidersSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`q`](ProvidersSearchQueryRequestBuilder::q)
    pub fn build(self) -> Result<ProvidersSearchQueryRequest, BuildError> {
        Ok(ProvidersSearchQueryRequest {
            q: self.q.ok_or_else(|| BuildError::missing_field("q"))?,
            limit: self.limit,
        })
    }
}

