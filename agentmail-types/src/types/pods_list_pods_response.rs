pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsListPodsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `created_at` descending.
    #[serde(default)]
    pub pods: Vec<PodsPod>,
}

impl PodsListPodsResponse {
    pub fn builder() -> PodsListPodsResponseBuilder {
        <PodsListPodsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsListPodsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    pods: Option<Vec<PodsPod>>,
}

impl PodsListPodsResponseBuilder {
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

    pub fn pods(mut self, value: Vec<PodsPod>) -> Self {
        self.pods = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodsListPodsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PodsListPodsResponseBuilder::count)
    /// - [`pods`](PodsListPodsResponseBuilder::pods)
    pub fn build(self) -> Result<PodsListPodsResponse, BuildError> {
        Ok(PodsListPodsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            pods: self.pods.ok_or_else(|| BuildError::missing_field("pods"))?,
        })
    }
}
