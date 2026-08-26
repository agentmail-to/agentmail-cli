pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tracked message was opened for the first time. Sent once per message: repeat opens do not
/// resend it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageOpenedEvent {
    pub r#type: MessageOpenedEventType,
    pub event_type: MessageOpenedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub open: Open,
}

impl MessageOpenedEvent {
    pub fn builder() -> MessageOpenedEventBuilder {
        <MessageOpenedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageOpenedEventBuilder {
    r#type: Option<MessageOpenedEventType>,
    event_type: Option<MessageOpenedEventEventType>,
    event_id: Option<EventId>,
    open: Option<Open>,
}

impl MessageOpenedEventBuilder {
    pub fn r#type(mut self, value: MessageOpenedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageOpenedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn open(mut self, value: Open) -> Self {
        self.open = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageOpenedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageOpenedEventBuilder::r#type)
    /// - [`event_type`](MessageOpenedEventBuilder::event_type)
    /// - [`event_id`](MessageOpenedEventBuilder::event_id)
    /// - [`open`](MessageOpenedEventBuilder::open)
    pub fn build(self) -> Result<MessageOpenedEvent, BuildError> {
        Ok(MessageOpenedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
        })
    }
}
