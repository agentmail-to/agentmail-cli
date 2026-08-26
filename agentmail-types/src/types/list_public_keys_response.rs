pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPublicKeysResponse {
    #[serde(default)]
    pub count: Count,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
    /// Public-key credentials only, ordered by creation time descending by default.
    #[serde(default)]
    pub public_keys: Vec<PublicKeyCredential>,
}

impl ListPublicKeysResponse {
    pub fn builder() -> ListPublicKeysResponseBuilder {
        <ListPublicKeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPublicKeysResponseBuilder {
    count: Option<Count>,
    next_page_token: Option<PageToken>,
    public_keys: Option<Vec<PublicKeyCredential>>,
}

impl ListPublicKeysResponseBuilder {
    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: PageToken) -> Self {
        self.next_page_token = Some(value);
        self
    }

    pub fn public_keys(mut self, value: Vec<PublicKeyCredential>) -> Self {
        self.public_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPublicKeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListPublicKeysResponseBuilder::count)
    /// - [`public_keys`](ListPublicKeysResponseBuilder::public_keys)
    pub fn build(self) -> Result<ListPublicKeysResponse, BuildError> {
        Ok(ListPublicKeysResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            next_page_token: self.next_page_token,
            public_keys: self.public_keys.ok_or_else(|| BuildError::missing_field("public_keys"))?,
        })
    }
}
