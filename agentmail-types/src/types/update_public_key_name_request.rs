pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePublicKeyNameRequest {
    #[serde(default)]
    pub name: String,
}

impl UpdatePublicKeyNameRequest {
    pub fn builder() -> UpdatePublicKeyNameRequestBuilder {
        <UpdatePublicKeyNameRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePublicKeyNameRequestBuilder {
    name: Option<String>,
}

impl UpdatePublicKeyNameRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePublicKeyNameRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdatePublicKeyNameRequestBuilder::name)
    pub fn build(self) -> Result<UpdatePublicKeyNameRequest, BuildError> {
        Ok(UpdatePublicKeyNameRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

