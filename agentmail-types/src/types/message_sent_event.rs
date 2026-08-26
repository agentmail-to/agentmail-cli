pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageSentEvent {
    pub r#type: MessageSentEventType,
    pub event_type: MessageSentEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub send: SendEvent,
}

impl MessageSentEvent {
    pub fn builder() -> MessageSentEventBuilder {
        <MessageSentEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageSentEventBuilder {
    r#type: Option<MessageSentEventType>,
    event_type: Option<MessageSentEventEventType>,
    event_id: Option<EventId>,
    send: Option<SendEvent>,
}

impl MessageSentEventBuilder {
    pub fn r#type(mut self, value: MessageSentEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageSentEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn send(mut self, value: SendEvent) -> Self {
        self.send = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageSentEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageSentEventBuilder::r#type)
    /// - [`event_type`](MessageSentEventBuilder::event_type)
    /// - [`event_id`](MessageSentEventBuilder::event_id)
    /// - [`send`](MessageSentEventBuilder::send)
    pub fn build(self) -> Result<MessageSentEvent, BuildError> {
        Ok(MessageSentEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            send: self.send.ok_or_else(|| BuildError::missing_field("send"))?,
        })
    }
}
