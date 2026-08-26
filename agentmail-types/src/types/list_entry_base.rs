pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEntryBase {
    /// Email address or domain of list entry.
    #[serde(default)]
    pub entry: String,
    #[serde(default)]
    pub organization_id: OrganizationId,
    /// Reason for adding the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub direction: Direction,
    pub list_type: ListType,
    pub entry_type: EntryType,
    /// Time at which entry was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Whether the entry is read-only and cannot be deleted via the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl ListEntryBase {
    pub fn builder() -> ListEntryBaseBuilder {
        <ListEntryBaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEntryBaseBuilder {
    entry: Option<String>,
    organization_id: Option<OrganizationId>,
    reason: Option<String>,
    direction: Option<Direction>,
    list_type: Option<ListType>,
    entry_type: Option<EntryType>,
    created_at: Option<DateTime<FixedOffset>>,
    read_only: Option<bool>,
}

impl ListEntryBaseBuilder {
    pub fn entry(mut self, value: impl Into<String>) -> Self {
        self.entry = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn list_type(mut self, value: ListType) -> Self {
        self.list_type = Some(value);
        self
    }

    pub fn entry_type(mut self, value: EntryType) -> Self {
        self.entry_type = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn read_only(mut self, value: bool) -> Self {
        self.read_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEntryBase`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry`](ListEntryBaseBuilder::entry)
    /// - [`organization_id`](ListEntryBaseBuilder::organization_id)
    /// - [`direction`](ListEntryBaseBuilder::direction)
    /// - [`list_type`](ListEntryBaseBuilder::list_type)
    /// - [`entry_type`](ListEntryBaseBuilder::entry_type)
    /// - [`created_at`](ListEntryBaseBuilder::created_at)
    pub fn build(self) -> Result<ListEntryBase, BuildError> {
        Ok(ListEntryBase {
            entry: self.entry.ok_or_else(|| BuildError::missing_field("entry"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            reason: self.reason,
            direction: self.direction.ok_or_else(|| BuildError::missing_field("direction"))?,
            list_type: self.list_type.ok_or_else(|| BuildError::missing_field("list_type"))?,
            entry_type: self.entry_type.ok_or_else(|| BuildError::missing_field("entry_type"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            read_only: self.read_only,
        })
    }
}
