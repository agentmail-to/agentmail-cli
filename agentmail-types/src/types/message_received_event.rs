pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A message was received. Spam, blocked, and unauthenticated received-message events use the same payload shape with different `event_type` values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReceivedEvent {
    pub r#type: MessageReceivedEventType,
    pub event_type: MessageReceivedEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub message: Message,
    #[serde(default)]
    pub thread: ThreadItem,
}

impl MessageReceivedEvent {
    pub fn builder() -> MessageReceivedEventBuilder {
        <MessageReceivedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageReceivedEventBuilder {
    r#type: Option<MessageReceivedEventType>,
    event_type: Option<MessageReceivedEventType>,
    event_id: Option<EventId>,
    message: Option<Message>,
    thread: Option<ThreadItem>,
}

impl MessageReceivedEventBuilder {
    pub fn r#type(mut self, value: MessageReceivedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageReceivedEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn message(mut self, value: Message) -> Self {
        self.message = Some(value);
        self
    }

    pub fn thread(mut self, value: ThreadItem) -> Self {
        self.thread = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageReceivedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageReceivedEventBuilder::r#type)
    /// - [`event_type`](MessageReceivedEventBuilder::event_type)
    /// - [`event_id`](MessageReceivedEventBuilder::event_id)
    /// - [`message`](MessageReceivedEventBuilder::message)
    /// - [`thread`](MessageReceivedEventBuilder::thread)
    pub fn build(self) -> Result<MessageReceivedEvent, BuildError> {
        Ok(MessageReceivedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            thread: self.thread.ok_or_else(|| BuildError::missing_field("thread"))?,
        })
    }
}
