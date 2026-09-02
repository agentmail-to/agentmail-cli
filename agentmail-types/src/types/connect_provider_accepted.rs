pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectProviderAccepted {
    /// ID of session.
    #[serde(default)]
    pub session_id: String,
    /// Single-use URL to open in the browser that will hold the sign-in.
    #[serde(default)]
    pub magic_url: String,
    /// Time at which the URL expires.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl ConnectProviderAccepted {
    pub fn builder() -> ConnectProviderAcceptedBuilder {
        <ConnectProviderAcceptedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectProviderAcceptedBuilder {
    session_id: Option<String>,
    magic_url: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl ConnectProviderAcceptedBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn magic_url(mut self, value: impl Into<String>) -> Self {
        self.magic_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectProviderAccepted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](ConnectProviderAcceptedBuilder::session_id)
    /// - [`magic_url`](ConnectProviderAcceptedBuilder::magic_url)
    /// - [`expires_at`](ConnectProviderAcceptedBuilder::expires_at)
    pub fn build(self) -> Result<ConnectProviderAccepted, BuildError> {
        Ok(ConnectProviderAccepted {
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            magic_url: self.magic_url.ok_or_else(|| BuildError::missing_field("magic_url"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
