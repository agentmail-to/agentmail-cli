pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The message an Agent Armor result is for. Fetch the message for its content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageSecurity {
    #[serde(default)]
    pub organization_id: OrganizationId,
    #[serde(default)]
    pub inbox_id: InboxesInboxId,
    #[serde(default)]
    pub message_id: MessageId,
    pub agent_armor: AgentArmorCompletedEventResult,
}

impl MessageSecurity {
    pub fn builder() -> MessageSecurityBuilder {
        <MessageSecurityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageSecurityBuilder {
    organization_id: Option<OrganizationId>,
    inbox_id: Option<InboxesInboxId>,
    message_id: Option<MessageId>,
    agent_armor: Option<AgentArmorCompletedEventResult>,
}

impl MessageSecurityBuilder {
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

    pub fn agent_armor(mut self, value: AgentArmorCompletedEventResult) -> Self {
        self.agent_armor = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageSecurity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](MessageSecurityBuilder::organization_id)
    /// - [`inbox_id`](MessageSecurityBuilder::inbox_id)
    /// - [`message_id`](MessageSecurityBuilder::message_id)
    /// - [`agent_armor`](MessageSecurityBuilder::agent_armor)
    pub fn build(self) -> Result<MessageSecurity, BuildError> {
        Ok(MessageSecurity {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            inbox_id: self.inbox_id.ok_or_else(|| BuildError::missing_field("inbox_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            agent_armor: self.agent_armor.ok_or_else(|| BuildError::missing_field("agent_armor"))?,
        })
    }
}
