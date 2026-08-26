pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Organization-wide authority.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationPublicKeyScope {
}

impl OrganizationPublicKeyScope {
    pub fn builder() -> OrganizationPublicKeyScopeBuilder {
        <OrganizationPublicKeyScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationPublicKeyScopeBuilder {
}

impl OrganizationPublicKeyScopeBuilder {

    /// Consumes the builder and constructs a [`OrganizationPublicKeyScope`].
    pub fn build(self) -> Result<OrganizationPublicKeyScope, BuildError> {
        Ok(OrganizationPublicKeyScope {
        })
    }
}
