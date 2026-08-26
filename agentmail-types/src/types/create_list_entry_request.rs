pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateListEntryRequest {
    /// Email address or domain to add.
    #[serde(default)]
    pub entry: String,
    /// Reason for adding the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CreateListEntryRequest {
    pub fn builder() -> CreateListEntryRequestBuilder {
        <CreateListEntryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateListEntryRequestBuilder {
    entry: Option<String>,
    reason: Option<String>,
}

impl CreateListEntryRequestBuilder {
    pub fn entry(mut self, value: impl Into<String>) -> Self {
        self.entry = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateListEntryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry`](CreateListEntryRequestBuilder::entry)
    pub fn build(self) -> Result<CreateListEntryRequest, BuildError> {
        Ok(CreateListEntryRequest {
            entry: self.entry.ok_or_else(|| BuildError::missing_field("entry"))?,
            reason: self.reason,
        })
    }
}
