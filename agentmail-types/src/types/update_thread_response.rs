pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateThreadResponse {
    #[serde(default)]
    pub thread_id: ThreadId,
    #[serde(default)]
    pub labels: ThreadLabels,
}

impl UpdateThreadResponse {
    pub fn builder() -> UpdateThreadResponseBuilder {
        <UpdateThreadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateThreadResponseBuilder {
    thread_id: Option<ThreadId>,
    labels: Option<ThreadLabels>,
}

impl UpdateThreadResponseBuilder {
    pub fn thread_id(mut self, value: ThreadId) -> Self {
        self.thread_id = Some(value);
        self
    }

    pub fn labels(mut self, value: ThreadLabels) -> Self {
        self.labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateThreadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_id`](UpdateThreadResponseBuilder::thread_id)
    /// - [`labels`](UpdateThreadResponseBuilder::labels)
    pub fn build(self) -> Result<UpdateThreadResponse, BuildError> {
        Ok(UpdateThreadResponse {
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
        })
    }
}
