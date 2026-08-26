pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InboxesThreadsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<PageToken>,
    #[serde(default)]
    pub labels: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Before>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<After>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<Ascending>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_spam: Option<IncludeSpam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocked: Option<IncludeBlocked>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unauthenticated: Option<IncludeUnauthenticated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_trash: Option<IncludeTrash>,
    /// Filter to threads whose senders contain this value (substring match). Repeatable; all values must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senders: Option<Vec<String>>,
    /// Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    /// Filter to threads whose subject contains this value (substring match). Repeatable; all values must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
}

impl InboxesThreadsListQueryRequest {
    pub fn builder() -> InboxesThreadsListQueryRequestBuilder {
        <InboxesThreadsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboxesThreadsListQueryRequestBuilder {
    limit: Option<Limit>,
    page_token: Option<PageToken>,
    labels: Option<Vec<Option<String>>>,
    before: Option<Before>,
    after: Option<After>,
    ascending: Option<Ascending>,
    include_spam: Option<IncludeSpam>,
    include_blocked: Option<IncludeBlocked>,
    include_unauthenticated: Option<IncludeUnauthenticated>,
    include_trash: Option<IncludeTrash>,
    senders: Option<Vec<String>>,
    recipients: Option<Vec<String>>,
    subject: Option<Vec<String>>,
}

impl InboxesThreadsListQueryRequestBuilder {
    pub fn limit(mut self, value: Limit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_token(mut self, value: PageToken) -> Self {
        self.page_token = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<Option<String>>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn before(mut self, value: Before) -> Self {
        self.before = Some(value);
        self
    }

    pub fn after(mut self, value: After) -> Self {
        self.after = Some(value);
        self
    }

    pub fn ascending(mut self, value: Ascending) -> Self {
        self.ascending = Some(value);
        self
    }

    pub fn include_spam(mut self, value: IncludeSpam) -> Self {
        self.include_spam = Some(value);
        self
    }

    pub fn include_blocked(mut self, value: IncludeBlocked) -> Self {
        self.include_blocked = Some(value);
        self
    }

    pub fn include_unauthenticated(mut self, value: IncludeUnauthenticated) -> Self {
        self.include_unauthenticated = Some(value);
        self
    }

    pub fn include_trash(mut self, value: IncludeTrash) -> Self {
        self.include_trash = Some(value);
        self
    }

    pub fn senders(mut self, value: Vec<String>) -> Self {
        self.senders = Some(value);
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

    /// Consumes the builder and constructs a [`InboxesThreadsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`labels`](InboxesThreadsListQueryRequestBuilder::labels)
    pub fn build(self) -> Result<InboxesThreadsListQueryRequest, BuildError> {
        Ok(InboxesThreadsListQueryRequest {
            limit: self.limit,
            page_token: self.page_token,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            before: self.before,
            after: self.after,
            ascending: self.ascending,
            include_spam: self.include_spam,
            include_blocked: self.include_blocked,
            include_unauthenticated: self.include_unauthenticated,
            include_trash: self.include_trash,
            senders: self.senders,
            recipients: self.recipients,
            subject: self.subject,
        })
    }
}

