pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorUnavailableResult {
    pub unavailable_reason: AgentArmorUnavailableReason,
}

impl AgentArmorUnavailableResult {
    pub fn builder() -> AgentArmorUnavailableResultBuilder {
        <AgentArmorUnavailableResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorUnavailableResultBuilder {
    unavailable_reason: Option<AgentArmorUnavailableReason>,
}

impl AgentArmorUnavailableResultBuilder {
    pub fn unavailable_reason(mut self, value: AgentArmorUnavailableReason) -> Self {
        self.unavailable_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorUnavailableResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unavailable_reason`](AgentArmorUnavailableResultBuilder::unavailable_reason)
    pub fn build(self) -> Result<AgentArmorUnavailableResult, BuildError> {
        Ok(AgentArmorUnavailableResult {
            unavailable_reason: self.unavailable_reason.ok_or_else(|| BuildError::missing_field("unavailable_reason"))?,
        })
    }
}
