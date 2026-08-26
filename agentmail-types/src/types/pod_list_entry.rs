pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodListEntry {
    #[serde(flatten)]
    pub list_entry_base_fields: ListEntryBase,
    /// ID of pod.
    #[serde(default)]
    pub pod_id: String,
    /// ID of inbox, if entry is inbox-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
}

impl PodListEntry {
    pub fn builder() -> PodListEntryBuilder {
        <PodListEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodListEntryBuilder {
    list_entry_base_fields: Option<ListEntryBase>,
    pod_id: Option<String>,
    inbox_id: Option<String>,
}

impl PodListEntryBuilder {
    pub fn list_entry_base_fields(mut self, value: ListEntryBase) -> Self {
        self.list_entry_base_fields = Some(value);
        self
    }

    pub fn pod_id(mut self, value: impl Into<String>) -> Self {
        self.pod_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodListEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`list_entry_base_fields`](PodListEntryBuilder::list_entry_base_fields)
    /// - [`pod_id`](PodListEntryBuilder::pod_id)
    pub fn build(self) -> Result<PodListEntry, BuildError> {
        Ok(PodListEntry {
            list_entry_base_fields: self.list_entry_base_fields.ok_or_else(|| BuildError::missing_field("list_entry_base_fields"))?,
            pod_id: self.pod_id.ok_or_else(|| BuildError::missing_field("pod_id"))?,
            inbox_id: self.inbox_id,
        })
    }
}
