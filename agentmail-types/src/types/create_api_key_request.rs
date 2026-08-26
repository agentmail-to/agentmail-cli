pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
}

impl CreateApiKeyRequest {
    pub fn builder() -> CreateApiKeyRequestBuilder {
        <CreateApiKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyRequestBuilder {
    name: Option<Name>,
    permissions: Option<ApiKeyPermissions>,
}

impl CreateApiKeyRequestBuilder {
    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);
        self
    }

    pub fn permissions(mut self, value: ApiKeyPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyRequest`].
    pub fn build(self) -> Result<CreateApiKeyRequest, BuildError> {
        Ok(CreateApiKeyRequest {
            name: self.name,
            permissions: self.permissions,
        })
    }
}
