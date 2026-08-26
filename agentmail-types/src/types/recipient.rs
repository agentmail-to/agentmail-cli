pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Recipient {
    /// Recipient address.
    #[serde(default)]
    pub address: String,
    /// Recipient status.
    #[serde(default)]
    pub status: String,
}

impl Recipient {
    pub fn builder() -> RecipientBuilder {
        <RecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecipientBuilder {
    address: Option<String>,
    status: Option<String>,
}

impl RecipientBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Recipient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](RecipientBuilder::address)
    /// - [`status`](RecipientBuilder::status)
    pub fn build(self) -> Result<Recipient, BuildError> {
        Ok(Recipient {
            address: self.address.ok_or_else(|| BuildError::missing_field("address"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
