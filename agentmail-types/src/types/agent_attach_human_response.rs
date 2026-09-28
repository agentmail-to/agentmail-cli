pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response after attaching a human to an agent organization.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentAttachHumanResponse {
    /// Email address of the attached human.
    #[serde(default)]
    pub human_email: String,
    /// Next steps for the agent, in plain text.
    #[serde(default)]
    pub instructions: String,
}

impl AgentAttachHumanResponse {
    pub fn builder() -> AgentAttachHumanResponseBuilder {
        <AgentAttachHumanResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentAttachHumanResponseBuilder {
    human_email: Option<String>,
    instructions: Option<String>,
}

impl AgentAttachHumanResponseBuilder {
    pub fn human_email(mut self, value: impl Into<String>) -> Self {
        self.human_email = Some(value.into());
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentAttachHumanResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`human_email`](AgentAttachHumanResponseBuilder::human_email)
    /// - [`instructions`](AgentAttachHumanResponseBuilder::instructions)
    pub fn build(self) -> Result<AgentAttachHumanResponse, BuildError> {
        Ok(AgentAttachHumanResponse {
            human_email: self.human_email.ok_or_else(|| BuildError::missing_field("human_email"))?,
            instructions: self.instructions.ok_or_else(|| BuildError::missing_field("instructions"))?,
        })
    }
}
