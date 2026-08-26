pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateThreadRequest {
    /// Labels to add to thread. Cannot be system labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_labels: Option<Vec<String>>,
    /// Labels to remove from thread. Cannot be system labels. Takes priority over `add_labels` (in the event of duplicate labels passed in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_labels: Option<Vec<String>>,
}

impl UpdateThreadRequest {
    pub fn builder() -> UpdateThreadRequestBuilder {
        <UpdateThreadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateThreadRequestBuilder {
    add_labels: Option<Vec<String>>,
    remove_labels: Option<Vec<String>>,
}

impl UpdateThreadRequestBuilder {
    pub fn add_labels(mut self, value: Vec<String>) -> Self {
        self.add_labels = Some(value);
        self
    }

    pub fn remove_labels(mut self, value: Vec<String>) -> Self {
        self.remove_labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateThreadRequest`].
    pub fn build(self) -> Result<UpdateThreadRequest, BuildError> {
        Ok(UpdateThreadRequest {
            add_labels: self.add_labels,
            remove_labels: self.remove_labels,
        })
    }
}
