pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    #[serde(default)]
    pub name: ErrorName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorCode>,
    #[serde(default)]
    pub message: ErrorMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<ErrorFix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<ErrorDocs>,
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        <ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseBuilder {
    name: Option<ErrorName>,
    code: Option<ErrorCode>,
    message: Option<ErrorMessage>,
    fix: Option<ErrorFix>,
    docs: Option<ErrorDocs>,
}

impl ErrorResponseBuilder {
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

    pub fn fix(mut self, value: ErrorFix) -> Self {
        self.fix = Some(value);
        self
    }

    pub fn docs(mut self, value: ErrorDocs) -> Self {
        self.docs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ErrorResponseBuilder::name)
    /// - [`message`](ErrorResponseBuilder::message)
    pub fn build(self) -> Result<ErrorResponse, BuildError> {
        Ok(ErrorResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            fix: self.fix,
            docs: self.docs,
        })
    }
}
