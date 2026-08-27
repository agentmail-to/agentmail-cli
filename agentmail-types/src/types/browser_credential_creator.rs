pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserCredentialCreator {
    pub kind: BrowserCredentialCreatorKind,
    /// Bearer API key that authorized creation of the browser credential.
    #[serde(default)]
    pub api_key_id: String,
    /// Incarnation timestamp of the authorizing bearer API key.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl BrowserCredentialCreator {
    pub fn builder() -> BrowserCredentialCreatorBuilder {
        <BrowserCredentialCreatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserCredentialCreatorBuilder {
    kind: Option<BrowserCredentialCreatorKind>,
    api_key_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl BrowserCredentialCreatorBuilder {
    pub fn kind(mut self, value: BrowserCredentialCreatorKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserCredentialCreator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](BrowserCredentialCreatorBuilder::kind)
    /// - [`api_key_id`](BrowserCredentialCreatorBuilder::api_key_id)
    /// - [`created_at`](BrowserCredentialCreatorBuilder::created_at)
    pub fn build(self) -> Result<BrowserCredentialCreator, BuildError> {
        Ok(BrowserCredentialCreator {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
