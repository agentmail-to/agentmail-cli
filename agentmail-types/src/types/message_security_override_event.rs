pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Someone released a message Agent Armor held for review. Its `message.received` is sent to your agents as well,
/// and the two can arrive in either order.
/// Sent once per message and inbox; `event_id` is stable. Not sent for messages labeled `spam`, `blocked`,
/// `unauthenticated`, or `trash`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageSecurityOverrideEvent {
    pub r#type: MessageSecurityOverrideEventType,
    pub event_type: MessageSecurityOverrideEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    pub message: MessageSecurityOverride,
}

impl MessageSecurityOverrideEvent {
    pub fn builder() -> MessageSecurityOverrideEventBuilder {
        <MessageSecurityOverrideEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageSecurityOverrideEventBuilder {
    r#type: Option<MessageSecurityOverrideEventType>,
    event_type: Option<MessageSecurityOverrideEventEventType>,
    event_id: Option<EventId>,
    message: Option<MessageSecurityOverride>,
}

impl MessageSecurityOverrideEventBuilder {
    pub fn r#type(mut self, value: MessageSecurityOverrideEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageSecurityOverrideEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn message(mut self, value: MessageSecurityOverride) -> Self {
        self.message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageSecurityOverrideEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageSecurityOverrideEventBuilder::r#type)
    /// - [`event_type`](MessageSecurityOverrideEventBuilder::event_type)
    /// - [`event_id`](MessageSecurityOverrideEventBuilder::event_id)
    /// - [`message`](MessageSecurityOverrideEventBuilder::message)
    pub fn build(self) -> Result<MessageSecurityOverrideEvent, BuildError> {
        Ok(MessageSecurityOverrideEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
