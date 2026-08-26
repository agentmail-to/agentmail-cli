pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboxesCreateInboxRequest {
    /// Username of address. Randomly generated if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Domain of address. Must be a verified domain, or any subdomain of a
    /// verified domain that has subdomains enabled (e.g., `bot.example.com`).
    /// Defaults to `agentmail.to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<InboxesDisplayName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<InboxesClientId>,
    /// Custom metadata to attach to the inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InboxesMetadata>,
}

impl InboxesCreateInboxRequest {
    pub fn builder() -> InboxesCreateInboxRequestBuilder {
        <InboxesCreateInboxRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesCreateInboxRequestBuilder {
    username: Option<String>,
    domain: Option<String>,
    display_name: Option<InboxesDisplayName>,
    client_id: Option<InboxesClientId>,
    metadata: Option<InboxesMetadata>,
}

impl InboxesCreateInboxRequestBuilder {
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: InboxesDisplayName) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn client_id(mut self, value: InboxesClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn metadata(mut self, value: InboxesMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesCreateInboxRequest`].
    pub fn build(self) -> Result<InboxesCreateInboxRequest, BuildError> {
        Ok(InboxesCreateInboxRequest {
            username: self.username,
            domain: self.domain,
            display_name: self.display_name,
            client_id: self.client_id,
            metadata: self.metadata,
        })
    }
}
