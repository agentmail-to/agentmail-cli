pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserConsentLifecycleEvent {
    pub r#type: BrowserConsentLifecycleEventType,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub occurred_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub pod_id: String,
    pub actor: BrowserLifecycleActor,
    #[serde(default)]
    pub consent_id: String,
    pub client_type: BrowserConsentLifecycleEventClientType,
    #[serde(default)]
    pub client_id: String,
}

impl BrowserConsentLifecycleEvent {
    pub fn builder() -> BrowserConsentLifecycleEventBuilder {
        <BrowserConsentLifecycleEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserConsentLifecycleEventBuilder {
    r#type: Option<BrowserConsentLifecycleEventType>,
    trace_id: Option<String>,
    event_id: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    organization_id: Option<String>,
    pod_id: Option<String>,
    actor: Option<BrowserLifecycleActor>,
    consent_id: Option<String>,
    client_type: Option<BrowserConsentLifecycleEventClientType>,
    client_id: Option<String>,
}

impl BrowserConsentLifecycleEventBuilder {
    pub fn r#type(mut self, value: BrowserConsentLifecycleEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn pod_id(mut self, value: impl Into<String>) -> Self {
        self.pod_id = Some(value.into());
        self
    }

    pub fn actor(mut self, value: BrowserLifecycleActor) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn consent_id(mut self, value: impl Into<String>) -> Self {
        self.consent_id = Some(value.into());
        self
    }

    pub fn client_type(mut self, value: BrowserConsentLifecycleEventClientType) -> Self {
        self.client_type = Some(value);
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrowserConsentLifecycleEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](BrowserConsentLifecycleEventBuilder::r#type)
    /// - [`trace_id`](BrowserConsentLifecycleEventBuilder::trace_id)
    /// - [`event_id`](BrowserConsentLifecycleEventBuilder::event_id)
    /// - [`occurred_at`](BrowserConsentLifecycleEventBuilder::occurred_at)
    /// - [`organization_id`](BrowserConsentLifecycleEventBuilder::organization_id)
    /// - [`pod_id`](BrowserConsentLifecycleEventBuilder::pod_id)
    /// - [`actor`](BrowserConsentLifecycleEventBuilder::actor)
    /// - [`consent_id`](BrowserConsentLifecycleEventBuilder::consent_id)
    /// - [`client_type`](BrowserConsentLifecycleEventBuilder::client_type)
    /// - [`client_id`](BrowserConsentLifecycleEventBuilder::client_id)
    pub fn build(self) -> Result<BrowserConsentLifecycleEvent, BuildError> {
        Ok(BrowserConsentLifecycleEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            consent_id: self.consent_id.ok_or_else(|| BuildError::missing_field("consent_id"))?,
            client_type: self.client_type.ok_or_else(|| BuildError::missing_field("client_type"))?,
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
        })
    }
}
