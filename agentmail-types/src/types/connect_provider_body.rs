pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectProviderBody {
    /// Inbox to connect. Required unless the API key is scoped to an inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<InboxesInboxId>,
    /// Authorize the provider for this inbox, skipping the first-use disclosure page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize: Option<bool>,
}

impl ConnectProviderBody {
    pub fn builder() -> ConnectProviderBodyBuilder {
        <ConnectProviderBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectProviderBodyBuilder {
    inbox_id: Option<InboxesInboxId>,
    authorize: Option<bool>,
}

impl ConnectProviderBodyBuilder {
    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn authorize(mut self, value: bool) -> Self {
        self.authorize = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectProviderBody`].
    pub fn build(self) -> Result<ConnectProviderBody, BuildError> {
        Ok(ConnectProviderBody {
            inbox_id: self.inbox_id,
            authorize: self.authorize,
        })
    }
}

