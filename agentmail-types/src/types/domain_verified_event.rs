pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainVerifiedEvent {
    pub r#type: DomainVerifiedEventType,
    pub event_type: DomainVerifiedEventEventType,
    #[serde(default)]
    pub event_id: EventId,
    pub domain: Domain,
}

impl DomainVerifiedEvent {
    pub fn builder() -> DomainVerifiedEventBuilder {
        <DomainVerifiedEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainVerifiedEventBuilder {
    r#type: Option<DomainVerifiedEventType>,
    event_type: Option<DomainVerifiedEventEventType>,
    event_id: Option<EventId>,
    domain: Option<Domain>,
}

impl DomainVerifiedEventBuilder {
    pub fn r#type(mut self, value: DomainVerifiedEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_type(mut self, value: DomainVerifiedEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_id(mut self, value: EventId) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn domain(mut self, value: Domain) -> Self {
        self.domain = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainVerifiedEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](DomainVerifiedEventBuilder::r#type)
    /// - [`event_type`](DomainVerifiedEventBuilder::event_type)
    /// - [`event_id`](DomainVerifiedEventBuilder::event_id)
    /// - [`domain`](DomainVerifiedEventBuilder::domain)
    pub fn build(self) -> Result<DomainVerifiedEvent, BuildError> {
        Ok(DomainVerifiedEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
        })
    }
}
