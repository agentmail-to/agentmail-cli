pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesAuthorizeInboxRequest {
    #[serde(default)]
    pub auth_token: AuthToken,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_disclosure: Option<AcceptDisclosure>,
}

impl InboxesAuthorizeInboxRequest {
    pub fn builder() -> InboxesAuthorizeInboxRequestBuilder {
        <InboxesAuthorizeInboxRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesAuthorizeInboxRequestBuilder {
    auth_token: Option<AuthToken>,
    accept_disclosure: Option<AcceptDisclosure>,
}

impl InboxesAuthorizeInboxRequestBuilder {
    pub fn auth_token(mut self, value: AuthToken) -> Self {
        self.auth_token = Some(value);
        self
    }

    pub fn accept_disclosure(mut self, value: AcceptDisclosure) -> Self {
        self.accept_disclosure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesAuthorizeInboxRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_token`](InboxesAuthorizeInboxRequestBuilder::auth_token)
    pub fn build(self) -> Result<InboxesAuthorizeInboxRequest, BuildError> {
        Ok(InboxesAuthorizeInboxRequest {
            auth_token: self.auth_token.ok_or_else(|| BuildError::missing_field("auth_token"))?,
            accept_disclosure: self.accept_disclosure,
        })
    }
}

