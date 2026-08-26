pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsPod {
    #[serde(default)]
    pub pod_id: PodsPodId,
    #[serde(default)]
    pub name: PodsName,
    /// Time at which pod was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Time at which pod was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<PodsClientId>,
}

impl PodsPod {
    pub fn builder() -> PodsPodBuilder {
        <PodsPodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsPodBuilder {
    pod_id: Option<PodsPodId>,
    name: Option<PodsName>,
    updated_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    client_id: Option<PodsClientId>,
}

impl PodsPodBuilder {
    pub fn pod_id(mut self, value: PodsPodId) -> Self {
        self.pod_id = Some(value);
        self
    }

    pub fn name(mut self, value: PodsName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn client_id(mut self, value: PodsClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodsPod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pod_id`](PodsPodBuilder::pod_id)
    /// - [`name`](PodsPodBuilder::name)
    /// - [`updated_at`](PodsPodBuilder::updated_at)
    /// - [`created_at`](PodsPodBuilder::created_at)
    pub fn build(self) -> Result<PodsPod, BuildError> {
        Ok(PodsPod {
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            client_id: self.client_id,
        })
    }
}
