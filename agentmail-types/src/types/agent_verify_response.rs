pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response after successful agent verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentVerifyResponse {
    /// Whether the organization was verified.
    #[serde(default)]
    pub verified: bool,
}

impl AgentVerifyResponse {
    pub fn builder() -> AgentVerifyResponseBuilder {
        <AgentVerifyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVerifyResponseBuilder {
    verified: Option<bool>,
}

impl AgentVerifyResponseBuilder {
    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentVerifyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verified`](AgentVerifyResponseBuilder::verified)
    pub fn build(self) -> Result<AgentVerifyResponse, BuildError> {
        Ok(AgentVerifyResponse {
            verified: self.verified.ok_or_else(|| BuildError::missing_field("verified"))?,
        })
    }
}
