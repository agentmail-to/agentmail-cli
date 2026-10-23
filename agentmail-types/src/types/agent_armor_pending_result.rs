pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentArmorPendingResult {
    /// When the message was received. The deadline counts from here.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// When the scan gives up. A result still pending then reads as `unavailable` with `deadline_exceeded`.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub deadline_at: DateTime<FixedOffset>,
}

impl AgentArmorPendingResult {
    pub fn builder() -> AgentArmorPendingResultBuilder {
        <AgentArmorPendingResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorPendingResultBuilder {
    started_at: Option<DateTime<FixedOffset>>,
    deadline_at: Option<DateTime<FixedOffset>>,
}

impl AgentArmorPendingResultBuilder {
    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn deadline_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deadline_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorPendingResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`started_at`](AgentArmorPendingResultBuilder::started_at)
    /// - [`deadline_at`](AgentArmorPendingResultBuilder::deadline_at)
    pub fn build(self) -> Result<AgentArmorPendingResult, BuildError> {
        Ok(AgentArmorPendingResult {
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            deadline_at: self.deadline_at.ok_or_else(|| BuildError::missing_field("deadline_at"))?,
        })
    }
}
