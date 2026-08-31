pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VerificationRecord {
    /// The type of the DNS record.
    pub r#type: RecordType,
    /// The name or host of the record.
    #[serde(default)]
    pub name: String,
    /// The value of the record.
    #[serde(default)]
    pub value: String,
    /// The verification status of this specific record.
    pub status: RecordStatus,
    /// The priority of the MX record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// Why the record is INVALID, when known. `duplicate_records` means the expected value is present but extra records coexist at the same name; `value_mismatch` means a record exists but does not match the expected value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl VerificationRecord {
    pub fn builder() -> VerificationRecordBuilder {
        <VerificationRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerificationRecordBuilder {
    r#type: Option<RecordType>,
    name: Option<String>,
    value: Option<String>,
    status: Option<RecordStatus>,
    priority: Option<i64>,
    reason: Option<String>,
}

impl VerificationRecordBuilder {
    pub fn r#type(mut self, value: RecordType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn status(mut self, value: RecordStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerificationRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](VerificationRecordBuilder::r#type)
    /// - [`name`](VerificationRecordBuilder::name)
    /// - [`value`](VerificationRecordBuilder::value)
    /// - [`status`](VerificationRecordBuilder::status)
    pub fn build(self) -> Result<VerificationRecord, BuildError> {
        Ok(VerificationRecord {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            priority: self.priority,
            reason: self.reason,
        })
    }
}
