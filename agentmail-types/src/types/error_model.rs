pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Error {
    pub r#type: ErrorType,
    #[serde(default)]
    pub name: ErrorName,
    #[serde(default)]
    pub message: ErrorMessage,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    r#type: Option<ErrorType>,
    name: Option<ErrorName>,
    message: Option<ErrorMessage>,
}

impl ErrorBuilder {
    pub fn r#type(mut self, value: ErrorType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: ErrorName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn message(mut self, value: ErrorMessage) -> Self {
        self.message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ErrorBuilder::r#type)
    /// - [`name`](ErrorBuilder::name)
    /// - [`message`](ErrorBuilder::message)
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
