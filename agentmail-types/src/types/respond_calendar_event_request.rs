pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RespondCalendarEventRequest {
    pub status: RespondStatus,
    /// Comment to include with the response. At most 4,096 UTF-8 bytes, with no control characters
    /// other than tab, line feed and carriage return; a longer comment returns `400`. It is stored
    /// as the inbox's attendee `comment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// When `true` (default), emails the response (iCalendar `REPLY`) to the organizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_reply: Option<bool>,
}

impl RespondCalendarEventRequest {
    pub fn builder() -> RespondCalendarEventRequestBuilder {
        <RespondCalendarEventRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RespondCalendarEventRequestBuilder {
    status: Option<RespondStatus>,
    comment: Option<String>,
    send_reply: Option<bool>,
}

impl RespondCalendarEventRequestBuilder {
    pub fn status(mut self, value: RespondStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    pub fn send_reply(mut self, value: bool) -> Self {
        self.send_reply = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RespondCalendarEventRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RespondCalendarEventRequestBuilder::status)
    pub fn build(self) -> Result<RespondCalendarEventRequest, BuildError> {
        Ok(RespondCalendarEventRequest {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            comment: self.comment,
            send_reply: self.send_reply,
        })
    }
}

