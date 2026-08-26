pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageRejectedEvent {
    pub r#type: MessageRejectedEventType,
    pub event_type: MessageRejectedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub reject: Reject,
}

impl MessageRejectedEvent {
    pub fn builder() -> MessageRejectedEventBuilder {
        <MessageRejectedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageRejectedEventBuilder {
    r#type: Option<MessageRejectedEventType>,
    event_type: Option<MessageRejectedEventEventType>,
    event_id: Option<EventId>,
    reject: Option<Reject>,
}

impl MessageRejectedEventBuilder {
    pub fn r#type(mut self, value: MessageRejectedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageRejectedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn reject(mut self, value: Reject) -> Self {
        self.reject = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageRejectedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageRejectedEventBuilder::r#type)
    /// - [`event_type`](MessageRejectedEventBuilder::event_type)
    /// - [`event_id`](MessageRejectedEventBuilder::event_id)
    /// - [`reject`](MessageRejectedEventBuilder::reject)
    pub fn build(self) -> Result<MessageRejectedEvent, BuildError> {
        Ok(MessageRejectedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            reject: self.reject.ok_or_else(|| BuildError::missing_field("reject"))?,
        })
    }
}
