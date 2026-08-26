pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Subscribed {
    pub r#type: SubscribedType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<EventTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<InboxIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ids: Option<PodIds>,
}

impl Subscribed {
    pub fn builder() -> SubscribedBuilder {
        <SubscribedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribedBuilder {
    r#type: Option<SubscribedType>,
    event_types: Option<EventTypes>,
    inbox_ids: Option<InboxIds>,
    pod_ids: Option<PodIds>,
}

impl SubscribedBuilder {
    pub fn r#type(mut self, value: SubscribedType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_types(mut self, value: EventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn inbox_ids(mut self, value: InboxIds) -> Self {
        self.inbox_ids = Some(value);
        self
    }

    pub fn pod_ids(mut self, value: PodIds) -> Self {
        self.pod_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Subscribed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SubscribedBuilder::r#type)
    pub fn build(self) -> Result<Subscribed, BuildError> {
        Ok(Subscribed {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_types: self.event_types,
            inbox_ids: self.inbox_ids,
            pod_ids: self.pod_ids,
        })
    }
}
