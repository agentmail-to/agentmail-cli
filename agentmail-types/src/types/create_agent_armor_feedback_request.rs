pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAgentArmorFeedbackRequest {
    pub feedback: AgentArmorFeedbackValue,
    /// Anything else about the verdict, up to 2,048 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl CreateAgentArmorFeedbackRequest {
    pub fn builder() -> CreateAgentArmorFeedbackRequestBuilder {
        <CreateAgentArmorFeedbackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentArmorFeedbackRequestBuilder {
    feedback: Option<AgentArmorFeedbackValue>,
    notes: Option<String>,
}

impl CreateAgentArmorFeedbackRequestBuilder {
    pub fn feedback(mut self, value: AgentArmorFeedbackValue) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentArmorFeedbackRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feedback`](CreateAgentArmorFeedbackRequestBuilder::feedback)
    pub fn build(self) -> Result<CreateAgentArmorFeedbackRequest, BuildError> {
        Ok(CreateAgentArmorFeedbackRequest {
            feedback: self.feedback.ok_or_else(|| BuildError::missing_field("feedback"))?,
            notes: self.notes,
        })
    }
}

