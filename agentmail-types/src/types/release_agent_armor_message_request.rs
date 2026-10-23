pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseAgentArmorMessageRequest {
    /// Why the message is safe to release, up to 2,048 characters and not blank. Recorded on the message.
    #[serde(default)]
    pub reason: String,
}

impl ReleaseAgentArmorMessageRequest {
    pub fn builder() -> ReleaseAgentArmorMessageRequestBuilder {
        <ReleaseAgentArmorMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseAgentArmorMessageRequestBuilder {
    reason: Option<String>,
}

impl ReleaseAgentArmorMessageRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseAgentArmorMessageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](ReleaseAgentArmorMessageRequestBuilder::reason)
    pub fn build(self) -> Result<ReleaseAgentArmorMessageRequest, BuildError> {
        Ok(ReleaseAgentArmorMessageRequest {
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}

