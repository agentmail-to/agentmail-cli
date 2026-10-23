pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The received message. The body (`text`, `html`, `extracted_text`, and `extracted_html`) is included
/// inline only when it is small enough. Webhooks omit it when the stored body is larger than 64 KiB;
/// WebSocket events also omit it when the serialized event would be larger than 30 KiB. When the body
/// is omitted, `body_url` and `body_expires_at` are set instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceivedMessage {
    #[serde(flatten)]
    pub message_fields: Message,
    /// Pre-signed URL that downloads the stored body as a JSON object including `text`, `html`,
    /// `extracted_text`, and `extracted_html`. Present only when the body is omitted from the event.
    /// Valid until `body_expires_at`, one hour after the event was built. The body stays available
    /// through the get message endpoint after the URL expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_url: Option<String>,
    /// Time at which `body_url` expires. Present only with `body_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_expires_at: Option<DateTime<FixedOffset>>,
}

impl ReceivedMessage {
    pub fn builder() -> ReceivedMessageBuilder {
        <ReceivedMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceivedMessageBuilder {
    message_fields: Option<Message>,
    body_url: Option<String>,
    body_expires_at: Option<DateTime<FixedOffset>>,
}

impl ReceivedMessageBuilder {
    pub fn message_fields(mut self, value: Message) -> Self {
        self.message_fields = Some(value);
        self
    }

    pub fn body_url(mut self, value: impl Into<String>) -> Self {
        self.body_url = Some(value.into());
        self
    }

    pub fn body_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.body_expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceivedMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_fields`](ReceivedMessageBuilder::message_fields)
    pub fn build(self) -> Result<ReceivedMessage, BuildError> {
        Ok(ReceivedMessage {
            message_fields: self.message_fields.ok_or_else(|| BuildError::missing_field("message_fields"))?,
            body_url: self.body_url,
            body_expires_at: self.body_expires_at,
        })
    }
}
