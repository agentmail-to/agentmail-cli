pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectAppBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<ConnectInboxId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_disclosure: Option<AcceptDisclosure>,
}

impl ConnectAppBody {
    pub fn builder() -> ConnectAppBodyBuilder {
        <ConnectAppBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectAppBodyBuilder {
    inbox_id: Option<ConnectInboxId>,
    accept_disclosure: Option<AcceptDisclosure>,
}

impl ConnectAppBodyBuilder {
    pub fn inbox_id(mut self, value: ConnectInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn accept_disclosure(mut self, value: AcceptDisclosure) -> Self {
        self.accept_disclosure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectAppBody`].
    pub fn build(self) -> Result<ConnectAppBody, BuildError> {
        Ok(ConnectAppBody {
            inbox_id: self.inbox_id,
            accept_disclosure: self.accept_disclosure,
        })
    }
}

