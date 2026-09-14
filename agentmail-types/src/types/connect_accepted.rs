pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The pending sign-in key the client will activate. Poll Get API Key with `api_key_id` for `status`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectAccepted {
    #[serde(default)]
    pub api_key_id: ApiKeyId,
    #[serde(default)]
    pub magic_url: MagicUrl,
    #[serde(default)]
    pub expires_at: ExpiresAt,
}

impl ConnectAccepted {
    pub fn builder() -> ConnectAcceptedBuilder {
        <ConnectAcceptedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectAcceptedBuilder {
    api_key_id: Option<ApiKeyId>,
    magic_url: Option<MagicUrl>,
    expires_at: Option<ExpiresAt>,
}

impl ConnectAcceptedBuilder {
    pub fn api_key_id(mut self, value: ApiKeyId) -> Self {
        self.api_key_id = Some(value);
        self
    }

    pub fn magic_url(mut self, value: MagicUrl) -> Self {
        self.magic_url = Some(value);
        self
    }

    pub fn expires_at(mut self, value: ExpiresAt) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectAccepted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](ConnectAcceptedBuilder::api_key_id)
    /// - [`magic_url`](ConnectAcceptedBuilder::magic_url)
    /// - [`expires_at`](ConnectAcceptedBuilder::expires_at)
    pub fn build(self) -> Result<ConnectAccepted, BuildError> {
        Ok(ConnectAccepted {
            api_key_id: self.api_key_id.ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            magic_url: self.magic_url.ok_or_else(|| BuildError::missing_field("magic_url"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
