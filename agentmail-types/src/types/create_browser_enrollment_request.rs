pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBrowserEnrollmentRequest {
    #[serde(default)]
    pub transaction_jti: BrowserEnrollmentTransactionJti,
}

impl CreateBrowserEnrollmentRequest {
    pub fn builder() -> CreateBrowserEnrollmentRequestBuilder {
        <CreateBrowserEnrollmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBrowserEnrollmentRequestBuilder {
    transaction_jti: Option<BrowserEnrollmentTransactionJti>,
}

impl CreateBrowserEnrollmentRequestBuilder {
    pub fn transaction_jti(mut self, value: BrowserEnrollmentTransactionJti) -> Self {
        self.transaction_jti = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBrowserEnrollmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_jti`](CreateBrowserEnrollmentRequestBuilder::transaction_jti)
    pub fn build(self) -> Result<CreateBrowserEnrollmentRequest, BuildError> {
        Ok(CreateBrowserEnrollmentRequest {
            transaction_jti: self.transaction_jti.ok_or_else(|| BuildError::missing_field("transaction_jti"))?,
        })
    }
}

