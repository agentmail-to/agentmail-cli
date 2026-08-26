pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageDeliveredEvent {
    pub r#type: MessageDeliveredEventType,
    pub event_type: MessageDeliveredEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub delivery: Delivery,
}

impl MessageDeliveredEvent {
    pub fn builder() -> MessageDeliveredEventBuilder {
        <MessageDeliveredEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageDeliveredEventBuilder {
    r#type: Option<MessageDeliveredEventType>,
    event_type: Option<MessageDeliveredEventEventType>,
    event_id: Option<EventId>,
    delivery: Option<Delivery>,
}

impl MessageDeliveredEventBuilder {
    pub fn r#type(mut self, value: MessageDeliveredEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageDeliveredEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn delivery(mut self, value: Delivery) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageDeliveredEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageDeliveredEventBuilder::r#type)
    /// - [`event_type`](MessageDeliveredEventBuilder::event_type)
    /// - [`event_id`](MessageDeliveredEventBuilder::event_id)
    /// - [`delivery`](MessageDeliveredEventBuilder::delivery)
    pub fn build(self) -> Result<MessageDeliveredEvent, BuildError> {
        Ok(MessageDeliveredEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            delivery: self.delivery.ok_or_else(|| BuildError::missing_field("delivery"))?,
        })
    }
}
