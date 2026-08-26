pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchMessageItem {
    #[serde(flatten)]
    pub message_item_fields: MessageItem,
    /// Matched fragments per field. Present only when the query matched an indexed field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<SearchMessageHighlights>,
}

impl SearchMessageItem {
    pub fn builder() -> SearchMessageItemBuilder {
        <SearchMessageItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMessageItemBuilder {
    message_item_fields: Option<MessageItem>,
    highlights: Option<SearchMessageHighlights>,
}

impl SearchMessageItemBuilder {
    pub fn message_item_fields(mut self, value: MessageItem) -> Self {
        self.message_item_fields = Some(value);
        self
    }

    pub fn highlights(mut self, value: SearchMessageHighlights) -> Self {
        self.highlights = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchMessageItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_item_fields`](SearchMessageItemBuilder::message_item_fields)
    pub fn build(self) -> Result<SearchMessageItem, BuildError> {
        Ok(SearchMessageItem {
            message_item_fields: self.message_item_fields.ok_or_else(|| BuildError::missing_field("message_item_fields"))?,
            highlights: self.highlights,
        })
    }
}
