pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The authorized sign-in's key ID plus the agent's next step. The key itself
/// is read with Get API Key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InboxesAuthorizeInboxResponse {
    /// ID of the public key the client will activate.
    #[serde(default)]
    pub api_key_id: ApiKeyId,
    /// The agent's next step. Nothing further is required from the agent.
    pub instructions: InboxesAuthorizeInboxResponseInstructions,
}

impl InboxesAuthorizeInboxResponse {
    pub fn builder() -> InboxesAuthorizeInboxResponseBuilder {
        <InboxesAuthorizeInboxResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesAuthorizeInboxResponseBuilder {
    api_key_id: Option<ApiKeyId>,
    instructions: Option<InboxesAuthorizeInboxResponseInstructions>,
}

impl InboxesAuthorizeInboxResponseBuilder {
    pub fn api_key_id(mut self, value: ApiKeyId) -> Self {
        self.api_key_id = Some(value);
        self
    }

    pub fn instructions(mut self, value: InboxesAuthorizeInboxResponseInstructions) -> Self {
        self.instructions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesAuthorizeInboxResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](InboxesAuthorizeInboxResponseBuilder::api_key_id)
    /// - [`instructions`](InboxesAuthorizeInboxResponseBuilder::instructions)
    pub fn build(self) -> Result<InboxesAuthorizeInboxResponse, BuildError> {
        Ok(InboxesAuthorizeInboxResponse {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            instructions: self.instructions.ok_or_else(|| BuildError::missing_field("instructions"))?,
        })
    }
}
