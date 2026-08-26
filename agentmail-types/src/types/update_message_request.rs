pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMessageRequest {
    /// Label or labels to add to message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_labels: Option<UpdateMessageLabels>,
    /// Label or labels to remove from message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_labels: Option<UpdateMessageLabels>,
}

impl UpdateMessageRequest {
    pub fn builder() -> UpdateMessageRequestBuilder {
        <UpdateMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMessageRequestBuilder {
    add_labels: Option<UpdateMessageLabels>,
    remove_labels: Option<UpdateMessageLabels>,
}

impl UpdateMessageRequestBuilder {
    pub fn add_labels(mut self, value: UpdateMessageLabels) -> Self {
        self.add_labels = Some(value);
        self
    }

    pub fn remove_labels(mut self, value: UpdateMessageLabels) -> Self {
        self.remove_labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMessageRequest`].
    pub fn build(self) -> Result<UpdateMessageRequest, BuildError> {
        Ok(UpdateMessageRequest {
            add_labels: self.add_labels,
            remove_labels: self.remove_labels,
        })
    }
}
