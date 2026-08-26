pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchUpdateMessagesResponse {
    #[serde(default)]
    pub limit: Limit,
    #[serde(default)]
    pub count: Count,
    /// Updated messages with their new labels. Order matches `message_ids`
    /// in the request. Excluded ids are omitted, so `count` may be less than
    /// `limit`.
    #[serde(default)]
    pub updates: Vec<UpdateMessageResponse>,
}

impl BatchUpdateMessagesResponse {
    pub fn builder() -> BatchUpdateMessagesResponseBuilder {
        <BatchUpdateMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchUpdateMessagesResponseBuilder {
    limit: Option<Limit>,
    count: Option<Count>,
    updates: Option<Vec<UpdateMessageResponse>>,
}

impl BatchUpdateMessagesResponseBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn count(mut self, value: Count) -> Self {
        self.count = Some(value);
        self
    }

    pub fn updates(mut self, value: Vec<UpdateMessageResponse>) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchUpdateMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](BatchUpdateMessagesResponseBuilder::limit)
    /// - [`count`](BatchUpdateMessagesResponseBuilder::count)
    /// - [`updates`](BatchUpdateMessagesResponseBuilder::updates)
    pub fn build(self) -> Result<BatchUpdateMessagesResponse, BuildError> {
        Ok(BatchUpdateMessagesResponse {
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            updates: self.updates.ok_or_else(|| BuildError::missing_field("updates"))?,
        })
    }
}
