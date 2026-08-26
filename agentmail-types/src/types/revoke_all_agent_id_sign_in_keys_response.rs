pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Permanent idempotency receipt for an organization-wide AgentID sign-in key revocation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeAllAgentIdSignInKeysResponse {
    #[serde(default)]
    pub previous_generation: i64,
    #[serde(default)]
    pub current_generation: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub revoked_at: DateTime<FixedOffset>,
}

impl RevokeAllAgentIdSignInKeysResponse {
    pub fn builder() -> RevokeAllAgentIdSignInKeysResponseBuilder {
        <RevokeAllAgentIdSignInKeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeAllAgentIdSignInKeysResponseBuilder {
    previous_generation: Option<i64>,
    current_generation: Option<i64>,
    revoked_at: Option<DateTime<FixedOffset>>,
}

impl RevokeAllAgentIdSignInKeysResponseBuilder {
    pub fn previous_generation(mut self, value: i64) -> Self {
        self.previous_generation = Some(value);
        self
    }

    pub fn current_generation(mut self, value: i64) -> Self {
        self.current_generation = Some(value);
        self
    }

    pub fn revoked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeAllAgentIdSignInKeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_generation`](RevokeAllAgentIdSignInKeysResponseBuilder::previous_generation)
    /// - [`current_generation`](RevokeAllAgentIdSignInKeysResponseBuilder::current_generation)
    /// - [`revoked_at`](RevokeAllAgentIdSignInKeysResponseBuilder::revoked_at)
    pub fn build(self) -> Result<RevokeAllAgentIdSignInKeysResponse, BuildError> {
        Ok(RevokeAllAgentIdSignInKeysResponse {
            previous_generation: self.previous_generation.ok_or_else(|| BuildError::missing_field("previous_generation"))?,
            current_generation: self.current_generation.ok_or_else(|| BuildError::missing_field("current_generation"))?,
            revoked_at: self.revoked_at.ok_or_else(|| BuildError::missing_field("revoked_at"))?,
        })
    }
}
