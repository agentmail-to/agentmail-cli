pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentArmorFeedback {
    /// ID of the feedback.
    #[serde(default)]
    pub feedback_id: String,
    pub feedback: AgentArmorFeedbackValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub actor: AgentArmorFeedbackActor,
    /// The verdict the feedback is about.
    pub original_verdict: AgentArmorVerdict,
    /// The Agent Armor release that produced that verdict.
    #[serde(default)]
    pub model: String,
    /// When the feedback was recorded.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl AgentArmorFeedback {
    pub fn builder() -> AgentArmorFeedbackBuilder {
        <AgentArmorFeedbackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorFeedbackBuilder {
    feedback_id: Option<String>,
    feedback: Option<AgentArmorFeedbackValue>,
    notes: Option<String>,
    actor: Option<AgentArmorFeedbackActor>,
    original_verdict: Option<AgentArmorVerdict>,
    model: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl AgentArmorFeedbackBuilder {
    pub fn feedback_id(mut self, value: impl Into<String>) -> Self {
        self.feedback_id = Some(value.into());
        self
    }

    pub fn feedback(mut self, value: AgentArmorFeedbackValue) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn actor(mut self, value: AgentArmorFeedbackActor) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn original_verdict(mut self, value: AgentArmorVerdict) -> Self {
        self.original_verdict = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentArmorFeedback`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feedback_id`](AgentArmorFeedbackBuilder::feedback_id)
    /// - [`feedback`](AgentArmorFeedbackBuilder::feedback)
    /// - [`actor`](AgentArmorFeedbackBuilder::actor)
    /// - [`original_verdict`](AgentArmorFeedbackBuilder::original_verdict)
    /// - [`model`](AgentArmorFeedbackBuilder::model)
    /// - [`created_at`](AgentArmorFeedbackBuilder::created_at)
    pub fn build(self) -> Result<AgentArmorFeedback, BuildError> {
        Ok(AgentArmorFeedback {
            feedback_id: self.feedback_id.ok_or_else(|| BuildError::missing_field("feedback_id"))?,
            feedback: self.feedback.ok_or_else(|| BuildError::missing_field("feedback"))?,
            notes: self.notes,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            original_verdict: self.original_verdict.ok_or_else(|| BuildError::missing_field("original_verdict"))?,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
