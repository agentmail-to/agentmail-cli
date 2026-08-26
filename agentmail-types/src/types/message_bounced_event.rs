pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageBouncedEvent {
    pub r#type: MessageBouncedEventType,
    pub event_type: MessageBouncedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub bounce: Bounce,
}

impl MessageBouncedEvent {
    pub fn builder() -> MessageBouncedEventBuilder {
        <MessageBouncedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageBouncedEventBuilder {
    r#type: Option<MessageBouncedEventType>,
    event_type: Option<MessageBouncedEventEventType>,
    event_id: Option<EventId>,
    bounce: Option<Bounce>,
}

impl MessageBouncedEventBuilder {
    pub fn r#type(mut self, value: MessageBouncedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageBouncedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn bounce(mut self, value: Bounce) -> Self {
        self.bounce = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageBouncedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageBouncedEventBuilder::r#type)
    /// - [`event_type`](MessageBouncedEventBuilder::event_type)
    /// - [`event_id`](MessageBouncedEventBuilder::event_id)
    /// - [`bounce`](MessageBouncedEventBuilder::bounce)
    pub fn build(self) -> Result<MessageBouncedEvent, BuildError> {
        Ok(MessageBouncedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            bounce: self.bounce.ok_or_else(|| BuildError::missing_field("bounce"))?,
        })
    }
}
