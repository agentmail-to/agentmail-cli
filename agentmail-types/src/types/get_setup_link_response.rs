pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSetupLinkResponse {
    /// Whether one-click setup is available for this domain. `false` means the domain's DNS provider does not support Domain Connect (or does not carry the AgentMail template yet) — add the domain's `records` manually instead.
    #[serde(default)]
    pub supported: bool,
    /// Display name of the domain's DNS provider, for the setup button label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// The signed Domain Connect apply URL. Open it in a browser: the domain owner signs in at their DNS provider, reviews the records, and approves — the provider writes them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Suggested popup width from the provider, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Suggested popup height from the provider, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Opaque value echoed back on the provider's redirect. Store it before opening the URL and compare on return to tie the redirect to this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Set when the domain currently has another email provider's MX records (for example Google Workspace). Applying the template would replace them — warn before proceeding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicting_provider: Option<String>,
}

impl GetSetupLinkResponse {
    pub fn builder() -> GetSetupLinkResponseBuilder {
        <GetSetupLinkResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSetupLinkResponseBuilder {
    supported: Option<bool>,
    provider_name: Option<String>,
    url: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    state: Option<String>,
    conflicting_provider: Option<String>,
}

impl GetSetupLinkResponseBuilder {
    pub fn supported(mut self, value: bool) -> Self {
        self.supported = Some(value);
        self
    }

    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn conflicting_provider(mut self, value: impl Into<String>) -> Self {
        self.conflicting_provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSetupLinkResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supported`](GetSetupLinkResponseBuilder::supported)
    pub fn build(self) -> Result<GetSetupLinkResponse, BuildError> {
        Ok(GetSetupLinkResponse {
            supported: self.supported.ok_or_else(|| BuildError::missing_field("supported"))?,
            provider_name: self.provider_name,
            url: self.url,
            width: self.width,
            height: self.height,
            state: self.state,
            conflicting_provider: self.conflicting_provider,
        })
    }
}
