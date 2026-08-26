pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKeyScopeOne {
    #[serde(flatten)]
    pub pod_public_key_scope_fields: PodPublicKeyScope,
    pub r#type: PublicKeyScopeOneType,
}

impl PublicKeyScopeOne {
    pub fn builder() -> PublicKeyScopeOneBuilder {
        <PublicKeyScopeOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicKeyScopeOneBuilder {
    pod_public_key_scope_fields: Option<PodPublicKeyScope>,
    r#type: Option<PublicKeyScopeOneType>,
}

impl PublicKeyScopeOneBuilder {
    pub fn pod_public_key_scope_fields(mut self, value: PodPublicKeyScope) -> Self {
        self.pod_public_key_scope_fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: PublicKeyScopeOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicKeyScopeOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pod_public_key_scope_fields`](PublicKeyScopeOneBuilder::pod_public_key_scope_fields)
    /// - [`r#type`](PublicKeyScopeOneBuilder::r#type)
    pub fn build(self) -> Result<PublicKeyScopeOne, BuildError> {
        Ok(PublicKeyScopeOne {
            pod_public_key_scope_fields: self.pod_public_key_scope_fields.ok_or_else(|| BuildError::missing_field("pod_public_key_scope_fields"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
