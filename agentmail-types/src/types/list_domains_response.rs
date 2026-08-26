pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDomainsResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Ordered by `created_at` descending.
    #[serde(default)]
    pub domains: Vec<DomainItem>,
}

impl ListDomainsResponse {
    pub fn builder() -> ListDomainsResponseBuilder {
        <ListDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDomainsResponseBuilder {
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    domains: Option<Vec<DomainItem>>,
}

impl ListDomainsResponseBuilder {
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

    pub fn domains(mut self, value: Vec<DomainItem>) -> Self {
        self.domains = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListDomainsResponseBuilder::count)
    /// - [`domains`](ListDomainsResponseBuilder::domains)
    pub fn build(self) -> Result<ListDomainsResponse, BuildError> {
        Ok(ListDomainsResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit,
            next_page_token: self.next_page_token,
            domains: self.domains.ok_or_else(|| BuildError::missing_field("domains"))?,
        })
    }
}
