pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentVerifyRequest {
    /// 6-digit verification code sent to the human's email address.
    #[serde(default)]
    pub otp_code: String,
}

impl AgentVerifyRequest {
    pub fn builder() -> AgentVerifyRequestBuilder {
        <AgentVerifyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVerifyRequestBuilder {
    otp_code: Option<String>,
}

impl AgentVerifyRequestBuilder {
    pub fn otp_code(mut self, value: impl Into<String>) -> Self {
        self.otp_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentVerifyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`otp_code`](AgentVerifyRequestBuilder::otp_code)
    pub fn build(self) -> Result<AgentVerifyRequest, BuildError> {
        Ok(AgentVerifyRequest {
            otp_code: self.otp_code.ok_or_else(|| BuildError::missing_field("otp_code"))?,
        })
    }
}

