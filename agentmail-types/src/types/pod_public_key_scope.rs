pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Authority over one live pod and its inboxes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodPublicKeyScope {
    /// ID of the pod.
    #[serde(default)]
    pub id: String,
}

impl PodPublicKeyScope {
    pub fn builder() -> PodPublicKeyScopeBuilder {
        <PodPublicKeyScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodPublicKeyScopeBuilder {
    id: Option<String>,
}

impl PodPublicKeyScopeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodPublicKeyScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PodPublicKeyScopeBuilder::id)
    pub fn build(self) -> Result<PodPublicKeyScope, BuildError> {
        Ok(PodPublicKeyScope {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
