pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Pending enrollment receipt. The browser completes key creation and proof
/// on the existing AgentID page. This response contains no URL, token, or
/// navigation instruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BrowserEnrollmentAccepted {
    pub status: BrowserEnrollmentAcceptedStatus,
    #[serde(default)]
    pub enrollment_id: String,
    /// Unix timestamp after which the pending enrollment cannot be activated.
    #[serde(default)]
    pub expires_at: i64,
}

impl BrowserEnrollmentAccepted {
    pub fn builder() -> BrowserEnrollmentAcceptedBuilder {
        <BrowserEnrollmentAcceptedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserEnrollmentAcceptedBuilder {
    status: Option<BrowserEnrollmentAcceptedStatus>,
    enrollment_id: Option<String>,
    expires_at: Option<i64>,
}

impl BrowserEnrollmentAcceptedBuilder {
    pub fn status(mut self, value: BrowserEnrollmentAcceptedStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn enrollment_id(mut self, value: impl Into<String>) -> Self {
        self.enrollment_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: i64) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserEnrollmentAccepted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BrowserEnrollmentAcceptedBuilder::status)
    /// - [`enrollment_id`](BrowserEnrollmentAcceptedBuilder::enrollment_id)
    /// - [`expires_at`](BrowserEnrollmentAcceptedBuilder::expires_at)
    pub fn build(self) -> Result<BrowserEnrollmentAccepted, BuildError> {
        Ok(BrowserEnrollmentAccepted {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            enrollment_id: self.enrollment_id.ok_or_else(|| BuildError::missing_field("enrollment_id"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
