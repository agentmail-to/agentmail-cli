pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageComplainedEvent {
    pub r#type: MessageComplainedEventType,
    pub event_type: MessageComplainedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub complaint: Complaint,
}

impl MessageComplainedEvent {
    pub fn builder() -> MessageComplainedEventBuilder {
        <MessageComplainedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageComplainedEventBuilder {
    r#type: Option<MessageComplainedEventType>,
    event_type: Option<MessageComplainedEventEventType>,
    event_id: Option<EventId>,
    complaint: Option<Complaint>,
}

impl MessageComplainedEventBuilder {
    pub fn r#type(mut self, value: MessageComplainedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: MessageComplainedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn complaint(mut self, value: Complaint) -> Self {
        self.complaint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageComplainedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MessageComplainedEventBuilder::r#type)
    /// - [`event_type`](MessageComplainedEventBuilder::event_type)
    /// - [`event_id`](MessageComplainedEventBuilder::event_id)
    /// - [`complaint`](MessageComplainedEventBuilder::complaint)
    pub fn build(self) -> Result<MessageComplainedEvent, BuildError> {
        Ok(MessageComplainedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            complaint: self.complaint.ok_or_else(|| BuildError::missing_field("complaint"))?,
        })
    }
}
