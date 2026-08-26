pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ValidationErrorResponse {
    #[serde(default)]
    pub name: ErrorName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
    pub errors: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<ErrorFix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<ErrorDocs>,
}

impl ValidationErrorResponse {
    pub fn builder() -> ValidationErrorResponseBuilder {
        <ValidationErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationErrorResponseBuilder {
    name: Option<ErrorName>,
    code: Option<ErrorCode>,
    message: Option<ErrorMessage>,
    errors: Option<serde_json::Value>,
    fix: Option<ErrorFix>,
    docs: Option<ErrorDocs>,
}

impl ValidationErrorResponseBuilder {
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

    pub fn errors(mut self, value: serde_json::Value) -> Self {
        self.errors = Some(value);
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

    /// Consumes the builder and constructs a [`ValidationErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ValidationErrorResponseBuilder::name)
    /// - [`errors`](ValidationErrorResponseBuilder::errors)
    pub fn build(self) -> Result<ValidationErrorResponse, BuildError> {
        Ok(ValidationErrorResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            message: self.message,
            errors: self.errors.ok_or_else(|| BuildError::missing_field("errors"))?,
            fix: self.fix,
            docs: self.docs,
        })
    }
}
