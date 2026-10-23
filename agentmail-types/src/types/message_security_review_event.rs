pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// In hold mode, Agent Armor held a received message for review: your agents are not sent its `message.received`
/// until someone releases it. The message carries the `agent-armor-held` label until then. Sent once per message and
/// inbox; `event_id` is stable. Not sent for messages labeled `spam`, `blocked`, `unauthenticated`, or `trash`: list
/// held mail by the `agent-armor-held` label instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageSecurityReviewEvent {
    pub r#type: MessageSecurityReviewEventType,
    pub event_type: MessageSecurityReviewEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    pub message: MessageSecurity,
}

impl MessageSecurityReviewEvent {
    pub fn builder() -> MessageSecurityReviewEventBuilder {
        <MessageSecurityReviewEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageSecurityReviewEventBuilder {
    r#type: Option<MessageSecurityReviewEventType>,
    event_type: Option<MessageSecurityReviewEventEventType>,
    event_id: Option<EventId>,
    message: Option<MessageSecurity>,
}

impl MessageSecurityReviewEventBuilder {
    pub fn r#type(mut self, value: MessageSecurityReviewEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageSecurityReviewEventEventType) -> Self {
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

    /// Consumes the builder and constructs a [`MessageSecurityReviewEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageSecurityReviewEventBuilder::r#type)
    /// - [`event_type`](MessageSecurityReviewEventBuilder::event_type)
    /// - [`event_id`](MessageSecurityReviewEventBuilder::event_id)
    /// - [`message`](MessageSecurityReviewEventBuilder::message)
    pub fn build(self) -> Result<MessageSecurityReviewEvent, BuildError> {
        Ok(MessageSecurityReviewEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
