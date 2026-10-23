pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreconditionFailedErrorResponse {
    #[serde(default)]
    pub name: ErrorName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorCode>,
    #[serde(default)]
    pub message: ErrorMessage,
    /// The resource's current revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<ErrorFix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<ErrorDocs>,
}

impl PreconditionFailedErrorResponse {
    pub fn builder() -> PreconditionFailedErrorResponseBuilder {
        <PreconditionFailedErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreconditionFailedErrorResponseBuilder {
    name: Option<ErrorName>,
    code: Option<ErrorCode>,
    message: Option<ErrorMessage>,
    current_revision: Option<i64>,
    fix: Option<ErrorFix>,
    docs: Option<ErrorDocs>,
}

impl PreconditionFailedErrorResponseBuilder {
    pub fn name(mut self, value: ErrorName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn code(mut self, value: ErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: ErrorMessage) -> Self {
        self.message = Some(value);
        self
    }

    pub fn current_revision(mut self, value: i64) -> Self {
        self.current_revision = Some(value);
        self
    }

    pub fn fix(mut self, value: ErrorFix) -> Self {
        self.fix = Some(value);
        self
    }

    pub fn docs(mut self, value: ErrorDocs) -> Self {
        self.docs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreconditionFailedErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PreconditionFailedErrorResponseBuilder::name)
    /// - [`message`](PreconditionFailedErrorResponseBuilder::message)
    pub fn build(self) -> Result<PreconditionFailedErrorResponse, BuildError> {
        Ok(PreconditionFailedErrorResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            current_revision: self.current_revision,
            fix: self.fix,
            docs: self.docs,
        })
    }
}
