pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKeyScopeZero {
    #[serde(flatten)]
    pub organization_public_key_scope_fields: OrganizationPublicKeyScope,
    pub r#type: PublicKeyScopeZeroType,
}

impl PublicKeyScopeZero {
    pub fn builder() -> PublicKeyScopeZeroBuilder {
        <PublicKeyScopeZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicKeyScopeZeroBuilder {
    organization_public_key_scope_fields: Option<OrganizationPublicKeyScope>,
    r#type: Option<PublicKeyScopeZeroType>,
}

impl PublicKeyScopeZeroBuilder {
    pub fn organization_public_key_scope_fields(mut self, value: OrganizationPublicKeyScope) -> Self {
        self.organization_public_key_scope_fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: PublicKeyScopeZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicKeyScopeZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_public_key_scope_fields`](PublicKeyScopeZeroBuilder::organization_public_key_scope_fields)
    /// - [`r#type`](PublicKeyScopeZeroBuilder::r#type)
    pub fn build(self) -> Result<PublicKeyScopeZero, BuildError> {
        Ok(PublicKeyScopeZero {
            organization_public_key_scope_fields: self.organization_public_key_scope_fields.ok_or_else(|| BuildError::missing_field("organization_public_key_scope_fields"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
