pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectProviderBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<ConnectInboxId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_disclosure: Option<AcceptDisclosure>,
}

impl ConnectProviderBody {
    pub fn builder() -> ConnectProviderBodyBuilder {
        <ConnectProviderBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectProviderBodyBuilder {
    inbox_id: Option<ConnectInboxId>,
    accept_disclosure: Option<AcceptDisclosure>,
}

impl ConnectProviderBodyBuilder {
    pub fn inbox_id(mut self, value: ConnectInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn accept_disclosure(mut self, value: AcceptDisclosure) -> Self {
        self.accept_disclosure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectProviderBody`].
    pub fn build(self) -> Result<ConnectProviderBody, BuildError> {
        Ok(ConnectProviderBody {
            inbox_id: self.inbox_id,
            accept_disclosure: self.accept_disclosure,
        })
    }
}

