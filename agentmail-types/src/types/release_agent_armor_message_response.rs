pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReleaseAgentArmorMessageResponse {
    pub manual_override: AgentArmorManualOverride,
}

impl ReleaseAgentArmorMessageResponse {
    pub fn builder() -> ReleaseAgentArmorMessageResponseBuilder {
        <ReleaseAgentArmorMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseAgentArmorMessageResponseBuilder {
    manual_override: Option<AgentArmorManualOverride>,
}

impl ReleaseAgentArmorMessageResponseBuilder {
    pub fn manual_override(mut self, value: AgentArmorManualOverride) -> Self {
        self.manual_override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleaseAgentArmorMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`manual_override`](ReleaseAgentArmorMessageResponseBuilder::manual_override)
    pub fn build(self) -> Result<ReleaseAgentArmorMessageResponse, BuildError> {
        Ok(ReleaseAgentArmorMessageResponse {
            manual_override: self.manual_override.ok_or_else(|| BuildError::missing_field("manual_override"))?,
        })
    }
}
