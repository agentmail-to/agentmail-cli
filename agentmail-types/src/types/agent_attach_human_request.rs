pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentAttachHumanRequest {
    /// Email address of the human who owns the agent. A 6-digit OTP will be sent to this address.
    #[serde(default)]
    pub human_email: String,
}

impl AgentAttachHumanRequest {
    pub fn builder() -> AgentAttachHumanRequestBuilder {
        <AgentAttachHumanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentAttachHumanRequestBuilder {
    human_email: Option<String>,
}

impl AgentAttachHumanRequestBuilder {
    pub fn human_email(mut self, value: impl Into<String>) -> Self {
        self.human_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentAttachHumanRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`human_email`](AgentAttachHumanRequestBuilder::human_email)
    pub fn build(self) -> Result<AgentAttachHumanRequest, BuildError> {
        Ok(AgentAttachHumanRequest {
            human_email: self.human_email.ok_or_else(|| BuildError::missing_field("human_email"))?,
        })
    }
}

