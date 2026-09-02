pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProviderAccountsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub accounts: Vec<Account>,
}

impl ListProviderAccountsResponse {
    pub fn builder() -> ListProviderAccountsResponseBuilder {
        <ListProviderAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProviderAccountsResponseBuilder {
    provider: Option<Provider>,
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    accounts: Option<Vec<Account>>,
}

impl ListProviderAccountsResponseBuilder {
    pub fn provider(mut self, value: Provider) -> Self {
        self.provider = Some(value);
        self
    }

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

    pub fn accounts(mut self, value: Vec<Account>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProviderAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListProviderAccountsResponseBuilder::count)
    /// - [`limit`](ListProviderAccountsResponseBuilder::limit)
    /// - [`accounts`](ListProviderAccountsResponseBuilder::accounts)
    pub fn build(self) -> Result<ListProviderAccountsResponse, BuildError> {
        Ok(ListProviderAccountsResponse {
            provider: self.provider,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            accounts: self.accounts.ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
