pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchThreadItem {
    #[serde(flatten)]
    pub thread_item_fields: ThreadItem,
    /// Matched fragments per field. Present only when the query matched an indexed field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<SearchThreadHighlights>,
}

impl SearchThreadItem {
    pub fn builder() -> SearchThreadItemBuilder {
        <SearchThreadItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchThreadItemBuilder {
    thread_item_fields: Option<ThreadItem>,
    highlights: Option<SearchThreadHighlights>,
}

impl SearchThreadItemBuilder {
    pub fn thread_item_fields(mut self, value: ThreadItem) -> Self {
        self.thread_item_fields = Some(value);
        self
    }

    pub fn highlights(mut self, value: SearchThreadHighlights) -> Self {
        self.highlights = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchThreadItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_item_fields`](SearchThreadItemBuilder::thread_item_fields)
    pub fn build(self) -> Result<SearchThreadItem, BuildError> {
        Ok(SearchThreadItem {
            thread_item_fields: self.thread_item_fields.ok_or_else(|| BuildError::missing_field("thread_item_fields"))?,
            highlights: self.highlights,
        })
    }
}
