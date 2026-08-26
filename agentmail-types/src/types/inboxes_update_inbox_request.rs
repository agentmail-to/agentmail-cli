pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboxesUpdateInboxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<InboxesDisplayName>,
    /// Metadata to merge into the inbox's existing metadata. Keys you include
    /// are added or overwritten; keys you omit are left unchanged. To remove a
    /// single key, send it with a null value. To clear all metadata, send
    /// `metadata` as null. Sending an empty object is rejected; use null to
    /// clear. Each update must include at least one of `display_name` or
    /// `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InboxesUpdateMetadata>,
}

impl InboxesUpdateInboxRequest {
    pub fn builder() -> InboxesUpdateInboxRequestBuilder {
        <InboxesUpdateInboxRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesUpdateInboxRequestBuilder {
    display_name: Option<InboxesDisplayName>,
    metadata: Option<InboxesUpdateMetadata>,
}

impl InboxesUpdateInboxRequestBuilder {
    pub fn display_name(mut self, value: InboxesDisplayName) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn metadata(mut self, value: InboxesUpdateMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InboxesUpdateInboxRequest`].
    pub fn build(self) -> Result<InboxesUpdateInboxRequest, BuildError> {
        Ok(InboxesUpdateInboxRequest {
            display_name: self.display_name,
            metadata: self.metadata,
        })
    }
}
