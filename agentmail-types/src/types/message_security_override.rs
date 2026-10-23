pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The message someone released, and the record of the release. Fetch the message for its content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageSecurityOverride {
    #[serde(default)]
    pub organization_id: OrganizationId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub message_id: MessageId,
    pub manual_override: AgentArmorManualOverride,
}

impl MessageSecurityOverride {
    pub fn builder() -> MessageSecurityOverrideBuilder {
        <MessageSecurityOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageSecurityOverrideBuilder {
    organization_id: Option<OrganizationId>,
    inbox_id: Option<InboxesInboxId>,
    message_id: Option<MessageId>,
    manual_override: Option<AgentArmorManualOverride>,
}

impl MessageSecurityOverrideBuilder {
    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn inbox_id(mut self, value: InboxesInboxId) -> Self {
        self.inbox_id = Some(value);
        self
    }

    pub fn message_id(mut self, value: MessageId) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn manual_override(mut self, value: AgentArmorManualOverride) -> Self {
        self.manual_override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageSecurityOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](MessageSecurityOverrideBuilder::organization_id)
    /// - [`inbox_id`](MessageSecurityOverrideBuilder::inbox_id)
    /// - [`message_id`](MessageSecurityOverrideBuilder::message_id)
    /// - [`manual_override`](MessageSecurityOverrideBuilder::manual_override)
    pub fn build(self) -> Result<MessageSecurityOverride, BuildError> {
        Ok(MessageSecurityOverride {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            manual_override: self.manual_override.ok_or_else(|| BuildError::missing_field("manual_override"))?,
        })
    }
}
