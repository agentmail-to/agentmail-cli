pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserEnrollmentLifecycleEvent {
    pub r#type: BrowserEnrollmentLifecycleEventType,
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
    pub enrollment_id: String,
    #[serde(default)]
    pub credential_id: String,
}

impl BrowserEnrollmentLifecycleEvent {
    pub fn builder() -> BrowserEnrollmentLifecycleEventBuilder {
        <BrowserEnrollmentLifecycleEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserEnrollmentLifecycleEventBuilder {
    r#type: Option<BrowserEnrollmentLifecycleEventType>,
    trace_id: Option<String>,
    event_id: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    organization_id: Option<String>,
    pod_id: Option<String>,
    actor: Option<BrowserLifecycleActor>,
    enrollment_id: Option<String>,
    credential_id: Option<String>,
}

impl BrowserEnrollmentLifecycleEventBuilder {
    pub fn r#type(mut self, value: BrowserEnrollmentLifecycleEventType) -> Self {
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

    pub fn enrollment_id(mut self, value: impl Into<String>) -> Self {
        self.enrollment_id = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrowserEnrollmentLifecycleEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](BrowserEnrollmentLifecycleEventBuilder::r#type)
    /// - [`trace_id`](BrowserEnrollmentLifecycleEventBuilder::trace_id)
    /// - [`event_id`](BrowserEnrollmentLifecycleEventBuilder::event_id)
    /// - [`occurred_at`](BrowserEnrollmentLifecycleEventBuilder::occurred_at)
    /// - [`organization_id`](BrowserEnrollmentLifecycleEventBuilder::organization_id)
    /// - [`pod_id`](BrowserEnrollmentLifecycleEventBuilder::pod_id)
    /// - [`actor`](BrowserEnrollmentLifecycleEventBuilder::actor)
    /// - [`enrollment_id`](BrowserEnrollmentLifecycleEventBuilder::enrollment_id)
    /// - [`credential_id`](BrowserEnrollmentLifecycleEventBuilder::credential_id)
    pub fn build(self) -> Result<BrowserEnrollmentLifecycleEvent, BuildError> {
        Ok(BrowserEnrollmentLifecycleEvent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            enrollment_id: self.enrollment_id.ok_or_else(|| BuildError::missing_field("enrollment_id"))?,
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
        })
    }
}
