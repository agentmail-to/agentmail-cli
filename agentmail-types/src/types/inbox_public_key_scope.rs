pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Authority over one live inbox incarnation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxPublicKeyScope {
    /// ID of the inbox.
    #[serde(default)]
    pub id: String,
}

impl InboxPublicKeyScope {
    pub fn builder() -> InboxPublicKeyScopeBuilder {
        <InboxPublicKeyScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxPublicKeyScopeBuilder {
    id: Option<String>,
}

impl InboxPublicKeyScopeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InboxPublicKeyScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InboxPublicKeyScopeBuilder::id)
    pub fn build(self) -> Result<InboxPublicKeyScope, BuildError> {
        Ok(InboxPublicKeyScope {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
