pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Matched fragments per field on a thread search result, with matched terms
/// wrapped in `**`. A field key is present only when the query matched that
/// field, so the present keys also tell you which fields produced the hit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchThreadHighlights {
    /// Matched fragments from a sender address in the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    /// Matched fragments from a recipient address in the thread (to, cc, or bcc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    /// Matched fragments from the subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
    /// Matched fragments from a message body in the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
}

impl SearchThreadHighlights {
    pub fn builder() -> SearchThreadHighlightsBuilder {
        <SearchThreadHighlightsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchThreadHighlightsBuilder {
    from: Option<Vec<String>>,
    recipients: Option<Vec<String>>,
    subject: Option<Vec<String>>,
    text: Option<Vec<String>>,
}

impl SearchThreadHighlightsBuilder {
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

    /// Consumes the builder and constructs a [`SearchThreadHighlights`].
    pub fn build(self) -> Result<SearchThreadHighlights, BuildError> {
        Ok(SearchThreadHighlights {
            from: self.from,
            recipients: self.recipients,
            subject: self.subject,
            text: self.text,
        })
    }
}
