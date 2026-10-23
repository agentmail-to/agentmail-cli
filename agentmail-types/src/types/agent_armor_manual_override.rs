pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The record of a release of a message Agent Armor held for review.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorManualOverride {
    /// ID of the release, and the `event_id` of its `message.security.override` event.
    #[serde(default)]
    pub override_id: String,
    pub actor: AgentArmorReleaseActor,
    /// Why it was released, as given by whoever released it.
    #[serde(default)]
    pub reason: String,
    /// The verdict that held the message.
    pub original_verdict: AgentArmorManualOverrideOriginalVerdict,
    /// When it was released.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub released_at: DateTime<FixedOffset>,
}

impl AgentArmorManualOverride {
    pub fn builder() -> AgentArmorManualOverrideBuilder {
        <AgentArmorManualOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorManualOverrideBuilder {
    override_id: Option<String>,
    actor: Option<AgentArmorReleaseActor>,
    reason: Option<String>,
    original_verdict: Option<AgentArmorManualOverrideOriginalVerdict>,
    released_at: Option<DateTime<FixedOffset>>,
}

impl AgentArmorManualOverrideBuilder {
    pub fn override_id(mut self, value: impl Into<String>) -> Self {
        self.override_id = Some(value.into());
        self
    }

    pub fn actor(mut self, value: AgentArmorReleaseActor) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn original_verdict(mut self, value: AgentArmorManualOverrideOriginalVerdict) -> Self {
        self.original_verdict = Some(value);
        self
    }

    pub fn released_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.released_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorManualOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`override_id`](AgentArmorManualOverrideBuilder::override_id)
    /// - [`actor`](AgentArmorManualOverrideBuilder::actor)
    /// - [`reason`](AgentArmorManualOverrideBuilder::reason)
    /// - [`original_verdict`](AgentArmorManualOverrideBuilder::original_verdict)
    /// - [`released_at`](AgentArmorManualOverrideBuilder::released_at)
    pub fn build(self) -> Result<AgentArmorManualOverride, BuildError> {
        Ok(AgentArmorManualOverride {
            override_id: self.override_id.ok_or_else(|| BuildError::missing_field("override_id"))?,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            original_verdict: self.original_verdict.ok_or_else(|| BuildError::missing_field("original_verdict"))?,
            released_at: self.released_at.ok_or_else(|| BuildError::missing_field("released_at"))?,
        })
    }
}
