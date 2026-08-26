pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKeyScopeTwo {
    #[serde(flatten)]
    pub inbox_public_key_scope_fields: InboxPublicKeyScope,
    pub r#type: PublicKeyScopeTwoType,
}

impl PublicKeyScopeTwo {
    pub fn builder() -> PublicKeyScopeTwoBuilder {
        <PublicKeyScopeTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicKeyScopeTwoBuilder {
    inbox_public_key_scope_fields: Option<InboxPublicKeyScope>,
    r#type: Option<PublicKeyScopeTwoType>,
}

impl PublicKeyScopeTwoBuilder {
    pub fn inbox_public_key_scope_fields(mut self, value: InboxPublicKeyScope) -> Self {
        self.inbox_public_key_scope_fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: PublicKeyScopeTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicKeyScopeTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inbox_public_key_scope_fields`](PublicKeyScopeTwoBuilder::inbox_public_key_scope_fields)
    /// - [`r#type`](PublicKeyScopeTwoBuilder::r#type)
    pub fn build(self) -> Result<PublicKeyScopeTwo, BuildError> {
        Ok(PublicKeyScopeTwo {
            inbox_public_key_scope_fields: self.inbox_public_key_scope_fields.ok_or_else(|| BuildError::missing_field("inbox_public_key_scope_fields"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
