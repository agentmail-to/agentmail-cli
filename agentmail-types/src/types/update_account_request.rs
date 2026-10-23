pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateAccountStatus>,
}

impl UpdateAccountRequest {
    pub fn builder() -> UpdateAccountRequestBuilder {
        <UpdateAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountRequestBuilder {
    status: Option<UpdateAccountStatus>,
}

impl UpdateAccountRequestBuilder {
    pub fn status(mut self, value: UpdateAccountStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountRequest`].
    pub fn build(self) -> Result<UpdateAccountRequest, BuildError> {
        Ok(UpdateAccountRequest {
            status: self.status,
        })
    }
}

