pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Attendee {
    /// Email address of the attendee. Stored in lowercase. Must be unique within the event.
    #[serde(default)]
    pub email: String,
    /// Display name of the attendee. At most 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Response status. Defaults to `needs_action`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AttendeeStatus>,
    /// Defaults to `required`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AttendeeRole>,
    /// Comment the attendee sent with their response. At most 4,096 UTF-8 bytes; on create or
    /// update, a longer comment is dropped rather than rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Time at which the attendee last responded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responded_at: Option<DateTime<FixedOffset>>,
}

impl Attendee {
    pub fn builder() -> AttendeeBuilder {
        <AttendeeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttendeeBuilder {
    email: Option<String>,
    name: Option<String>,
    status: Option<AttendeeStatus>,
    role: Option<AttendeeRole>,
    comment: Option<String>,
    responded_at: Option<DateTime<FixedOffset>>,
}

impl AttendeeBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: AttendeeStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn role(mut self, value: AttendeeRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    pub fn responded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.responded_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Attendee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](AttendeeBuilder::email)
    pub fn build(self) -> Result<Attendee, BuildError> {
        Ok(Attendee {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            status: self.status,
            role: self.role,
            comment: self.comment,
            responded_at: self.responded_at,
        })
    }
}
