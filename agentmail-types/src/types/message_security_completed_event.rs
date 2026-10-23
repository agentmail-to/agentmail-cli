pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Agent Armor reached a verdict on a received message. Sent only in monitor mode, and only for `complete` results:
/// an `unavailable` result sends no event. Sent once per message and inbox; `event_id` is stable, so use it to
/// deduplicate. Not sent for messages labeled `spam`, `blocked`, `unauthenticated`, or `trash`: read those through
/// the API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageSecurityCompletedEvent {
    pub r#type: MessageSecurityCompletedEventType,
    pub event_type: MessageSecurityCompletedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    pub message: MessageSecurity,
}

impl MessageSecurityCompletedEvent {
    pub fn builder() -> MessageSecurityCompletedEventBuilder {
        <MessageSecurityCompletedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageSecurityCompletedEventBuilder {
    r#type: Option<MessageSecurityCompletedEventType>,
    event_type: Option<MessageSecurityCompletedEventEventType>,
    event_id: Option<EventId>,
    message: Option<MessageSecurity>,
}

impl MessageSecurityCompletedEventBuilder {
    pub fn r#type(mut self, value: MessageSecurityCompletedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageSecurityCompletedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn message(mut self, value: MessageSecurity) -> Self {
        self.message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageSecurityCompletedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageSecurityCompletedEventBuilder::r#type)
    /// - [`event_type`](MessageSecurityCompletedEventBuilder::event_type)
    /// - [`event_id`](MessageSecurityCompletedEventBuilder::event_id)
    /// - [`message`](MessageSecurityCompletedEventBuilder::message)
    pub fn build(self) -> Result<MessageSecurityCompletedEvent, BuildError> {
        Ok(MessageSecurityCompletedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
