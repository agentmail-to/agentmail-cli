pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAppAccountsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
    #[serde(default)]
    pub count: Count,
    #[serde(default)]
    pub limit: Limit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    #[serde(default)]
    pub accounts: Vec<Account>,
}

impl ListAppAccountsResponse {
    pub fn builder() -> ListAppAccountsResponseBuilder {
        <ListAppAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAppAccountsResponseBuilder {
    app: Option<App>,
    count: Option<Count>,
    limit: Option<Limit>,
    next_page_token: Option<PageToken>,
    accounts: Option<Vec<Account>>,
}

impl ListAppAccountsResponseBuilder {
    pub fn app(mut self, value: App) -> Self {
        self.app = Some(value);
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

    /// Consumes the builder and constructs a [`ListAppAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListAppAccountsResponseBuilder::count)
    /// - [`limit`](ListAppAccountsResponseBuilder::limit)
    /// - [`accounts`](ListAppAccountsResponseBuilder::accounts)
    pub fn build(self) -> Result<ListAppAccountsResponse, BuildError> {
        Ok(ListAppAccountsResponse {
            app: self.app,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_page_token: self.next_page_token,
            accounts: self.accounts.ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
