pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response after successful agent sign-up.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSignupResponse {
    /// ID of the created organization.
    #[serde(default)]
    pub organization_id: String,
    /// ID of the auto-created inbox.
    #[serde(default)]
    pub inbox_id: String,
    /// API key for authenticating subsequent requests. Store this securely, it cannot be retrieved again.
    #[serde(default)]
    pub api_key: String,
}

impl AgentSignupResponse {
    pub fn builder() -> AgentSignupResponseBuilder {
        <AgentSignupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSignupResponseBuilder {
    organization_id: Option<String>,
    inbox_id: Option<String>,
    api_key: Option<String>,
}

impl AgentSignupResponseBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSignupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](AgentSignupResponseBuilder::organization_id)
    /// - [`inbox_id`](AgentSignupResponseBuilder::inbox_id)
    /// - [`api_key`](AgentSignupResponseBuilder::api_key)
    pub fn build(self) -> Result<AgentSignupResponse, BuildError> {
        Ok(AgentSignupResponse {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
