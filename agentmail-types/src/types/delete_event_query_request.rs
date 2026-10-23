pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete-event
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEventQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<InstanceMutationMode>,
    /// When `true`, emails a cancellation (iCalendar `CANCEL`) to every attendee. Only the organizer can send. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_invites: Option<bool>,
}

impl DeleteEventQueryRequest {
    pub fn builder() -> DeleteEventQueryRequestBuilder {
        <DeleteEventQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEventQueryRequestBuilder {
    mode: Option<InstanceMutationMode>,
    send_invites: Option<bool>,
}

impl DeleteEventQueryRequestBuilder {
    pub fn mode(mut self, value: InstanceMutationMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn send_invites(mut self, value: bool) -> Self {
        self.send_invites = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteEventQueryRequest`].
    pub fn build(self) -> Result<DeleteEventQueryRequest, BuildError> {
        Ok(DeleteEventQueryRequest {
            mode: self.mode,
            send_invites: self.send_invites,
        })
    }
}

