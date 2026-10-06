pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    MessageReceived,
    MessageReceivedSpam,
    MessageReceivedBlocked,
    MessageReceivedUnauthenticated,
    MessageSent,
    MessageDelivered,
    MessageBounced,
    MessageComplained,
    MessageRejected,
    MessageOpened,
    DomainVerified,
    CalendarEventCreated,
    CalendarEventUpdated,
    CalendarEventDeleted,
    CalendarEventResponded,
    CalendarEventStarting,
    CalendarEventEnding,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MessageReceived => serializer.serialize_str("message.received"),
            Self::MessageReceivedSpam => serializer.serialize_str("message.received.spam"),
            Self::MessageReceivedBlocked => serializer.serialize_str("message.received.blocked"),
            Self::MessageReceivedUnauthenticated => serializer.serialize_str("message.received.unauthenticated"),
            Self::MessageSent => serializer.serialize_str("message.sent"),
            Self::MessageDelivered => serializer.serialize_str("message.delivered"),
            Self::MessageBounced => serializer.serialize_str("message.bounced"),
            Self::MessageComplained => serializer.serialize_str("message.complained"),
            Self::MessageRejected => serializer.serialize_str("message.rejected"),
            Self::MessageOpened => serializer.serialize_str("message.opened"),
            Self::DomainVerified => serializer.serialize_str("domain.verified"),
            Self::CalendarEventCreated => serializer.serialize_str("calendar.event.created"),
            Self::CalendarEventUpdated => serializer.serialize_str("calendar.event.updated"),
            Self::CalendarEventDeleted => serializer.serialize_str("calendar.event.deleted"),
            Self::CalendarEventResponded => serializer.serialize_str("calendar.event.responded"),
            Self::CalendarEventStarting => serializer.serialize_str("calendar.event.starting"),
            Self::CalendarEventEnding => serializer.serialize_str("calendar.event.ending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "message.received" => Ok(Self::MessageReceived),
            "message.received.spam" => Ok(Self::MessageReceivedSpam),
            "message.received.blocked" => Ok(Self::MessageReceivedBlocked),
            "message.received.unauthenticated" => Ok(Self::MessageReceivedUnauthenticated),
            "message.sent" => Ok(Self::MessageSent),
            "message.delivered" => Ok(Self::MessageDelivered),
            "message.bounced" => Ok(Self::MessageBounced),
            "message.complained" => Ok(Self::MessageComplained),
            "message.rejected" => Ok(Self::MessageRejected),
            "message.opened" => Ok(Self::MessageOpened),
            "domain.verified" => Ok(Self::DomainVerified),
            "calendar.event.created" => Ok(Self::CalendarEventCreated),
            "calendar.event.updated" => Ok(Self::CalendarEventUpdated),
            "calendar.event.deleted" => Ok(Self::CalendarEventDeleted),
            "calendar.event.responded" => Ok(Self::CalendarEventResponded),
            "calendar.event.starting" => Ok(Self::CalendarEventStarting),
            "calendar.event.ending" => Ok(Self::CalendarEventEnding),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageReceived => write!(f, "message.received"),
            Self::MessageReceivedSpam => write!(f, "message.received.spam"),
            Self::MessageReceivedBlocked => write!(f, "message.received.blocked"),
            Self::MessageReceivedUnauthenticated => write!(f, "message.received.unauthenticated"),
            Self::MessageSent => write!(f, "message.sent"),
            Self::MessageDelivered => write!(f, "message.delivered"),
            Self::MessageBounced => write!(f, "message.bounced"),
            Self::MessageComplained => write!(f, "message.complained"),
            Self::MessageRejected => write!(f, "message.rejected"),
            Self::MessageOpened => write!(f, "message.opened"),
            Self::DomainVerified => write!(f, "domain.verified"),
            Self::CalendarEventCreated => write!(f, "calendar.event.created"),
            Self::CalendarEventUpdated => write!(f, "calendar.event.updated"),
            Self::CalendarEventDeleted => write!(f, "calendar.event.deleted"),
            Self::CalendarEventResponded => write!(f, "calendar.event.responded"),
            Self::CalendarEventStarting => write!(f, "calendar.event.starting"),
            Self::CalendarEventEnding => write!(f, "calendar.event.ending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
