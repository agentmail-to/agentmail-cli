pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Matched fragments per field on a message search result, with matched terms
/// wrapped in `**`. A field key is present only when the query matched that
/// field, so the present keys also tell you which fields produced the hit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchMessageHighlights {
    /// Matched fragments from the sender address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    /// Matched fragments from the recipient addresses (to, cc, or bcc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    /// Matched fragments from the subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
    /// Matched fragments from the message body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
}

impl SearchMessageHighlights {
    pub fn builder() -> SearchMessageHighlightsBuilder {
        <SearchMessageHighlightsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMessageHighlightsBuilder {
    from: Option<Vec<String>>,
    recipients: Option<Vec<String>>,
    subject: Option<Vec<String>>,
    text: Option<Vec<String>>,
}

impl SearchMessageHighlightsBuilder {
    pub fn from(mut self, value: Vec<String>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn subject(mut self, value: Vec<String>) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn text(mut self, value: Vec<String>) -> Self {
        self.text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchMessageHighlights`].
    pub fn build(self) -> Result<SearchMessageHighlights, BuildError> {
        Ok(SearchMessageHighlights {
            from: self.from,
            recipients: self.recipients,
            subject: self.subject,
            text: self.text,
        })
    }
}
