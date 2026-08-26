pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSignupRequest {
    /// Email address of the human who owns the agent. A 6-digit OTP will be sent to this address.
    #[serde(default)]
    pub human_email: String,
    /// Username for the auto-created inbox (e.g. "my-agent" creates my-agent@agentmail.to).
    #[serde(default)]
    pub username: String,
    /// The SDK, framework, or platform issuing this sign-up (e.g. `agentmail-python`, `agentmail-cli`, `agentmail-mcp`).
    /// Identifies the caller — answers "who is signing up".
    /// Max 2048 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The channel that drove this sign-up — where the agent or its developer discovered AgentMail
    /// (e.g. `agent.email`, a partner URL, a campaign tag). Answers "where did this sign-up come from".
    /// Max 2048 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
}

impl AgentSignupRequest {
    pub fn builder() -> AgentSignupRequestBuilder {
        <AgentSignupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSignupRequestBuilder {
    human_email: Option<String>,
    username: Option<String>,
    source: Option<String>,
    referrer: Option<String>,
}

impl AgentSignupRequestBuilder {
    pub fn human_email(mut self, value: impl Into<String>) -> Self {
        self.human_email = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn referrer(mut self, value: impl Into<String>) -> Self {
        self.referrer = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSignupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`human_email`](AgentSignupRequestBuilder::human_email)
    /// - [`username`](AgentSignupRequestBuilder::username)
    pub fn build(self) -> Result<AgentSignupRequest, BuildError> {
        Ok(AgentSignupRequest {
            human_email: self.human_email.ok_or_else(|| BuildError::missing_field("human_email"))?,
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
            source: self.source,
            referrer: self.referrer,
        })
    }
}

