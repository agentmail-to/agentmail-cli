pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorCompleteResult {
    pub verdict: AgentArmorVerdict,
    /// A short explanation of the verdict.
    #[serde(default)]
    pub reason: String,
    /// Identifies the Agent Armor release that produced the verdict. Opaque; it changes when Agent Armor is updated.
    #[serde(default)]
    pub model: String,
    /// When the scan finished.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub completed_at: DateTime<FixedOffset>,
    /// Present on a `review` held in hold mode that someone released.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_override: Option<AgentArmorManualOverride>,
}

impl AgentArmorCompleteResult {
    pub fn builder() -> AgentArmorCompleteResultBuilder {
        <AgentArmorCompleteResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorCompleteResultBuilder {
    verdict: Option<AgentArmorVerdict>,
    reason: Option<String>,
    model: Option<String>,
    completed_at: Option<DateTime<FixedOffset>>,
    manual_override: Option<AgentArmorManualOverride>,
}

impl AgentArmorCompleteResultBuilder {
    pub fn verdict(mut self, value: AgentArmorVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn manual_override(mut self, value: AgentArmorManualOverride) -> Self {
        self.manual_override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorCompleteResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verdict`](AgentArmorCompleteResultBuilder::verdict)
    /// - [`reason`](AgentArmorCompleteResultBuilder::reason)
    /// - [`model`](AgentArmorCompleteResultBuilder::model)
    /// - [`completed_at`](AgentArmorCompleteResultBuilder::completed_at)
    pub fn build(self) -> Result<AgentArmorCompleteResult, BuildError> {
        Ok(AgentArmorCompleteResult {
            verdict: self.verdict.ok_or_else(|| BuildError::missing_field("verdict"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            completed_at: self.completed_at.ok_or_else(|| BuildError::missing_field("completed_at"))?,
            manual_override: self.manual_override,
        })
    }
}
