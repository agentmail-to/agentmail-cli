pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The fields a caller may set on a bearer key at creation and change afterward.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyMutableFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
}

impl ApiKeyMutableFields {
    pub fn builder() -> ApiKeyMutableFieldsBuilder {
        <ApiKeyMutableFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyMutableFieldsBuilder {
    name: Option<Name>,
    permissions: Option<ApiKeyPermissions>,
}

impl ApiKeyMutableFieldsBuilder {
    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);
        self
    }

    pub fn permissions(mut self, value: ApiKeyPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyMutableFields`].
    pub fn build(self) -> Result<ApiKeyMutableFields, BuildError> {
        Ok(ApiKeyMutableFields {
            name: self.name,
            permissions: self.permissions,
        })
    }
}
