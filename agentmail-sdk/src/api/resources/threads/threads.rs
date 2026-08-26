use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ThreadsClient {
    pub http_client: HttpClient,
}

impl ThreadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists threads, most recent first. Pass `senders`, `recipients`, or
    /// `subject` to filter by substring. Filtered requests are served by
    /// search, which caps `limit` at 100. For relevance-ranked full-text
    /// search across senders, recipients, subject, and message body, use
    /// `Search Threads`.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail threads list
    /// ```
    ///
    /// # Arguments
    ///
    /// * `senders` - Filter to threads whose senders contain this value (substring match). Repeatable; all values must match.
    /// * `recipients` - Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match.
    /// * `subject` - Filter to threads whose subject contains this value (substring match). Repeatable; all values must match.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .threads
    ///         .list(
    ///             &ThreadsListQueryRequest {
    ///                 limit: None,
    ///                 page_token: None,
    ///                 labels: vec![],
    ///                 before: None,
    ///                 after: None,
    ///                 ascending: None,
    ///                 include_spam: None,
    ///                 include_blocked: None,
    ///                 include_unauthenticated: None,
    ///                 include_trash: None,
    ///                 senders: None,
    ///                 recipients: None,
    ///                 subject: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ThreadsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListThreadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/threads",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .string_array("labels", request.labels.clone())
                    .serialize("before", request.before.clone())
                    .serialize("after", request.after.clone())
                    .serialize("ascending", request.ascending.clone())
                    .serialize("include_spam", request.include_spam.clone())
                    .serialize("include_blocked", request.include_blocked.clone())
                    .serialize(
                        "include_unauthenticated",
                        request.include_unauthenticated.clone(),
                    )
                    .serialize("include_trash", request.include_trash.clone())
                    .serialize("senders", request.senders.clone())
                    .serialize("recipients", request.recipients.clone())
                    .serialize("subject", request.subject.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Full-text search across threads in the organization, ranked by
    /// relevance. The query is matched against senders, recipients, and
    /// subject (substring) and the message body (tokenized full text). Spam,
    /// trash, blocked, and unauthenticated threads are always excluded.
    /// `limit` cannot exceed 100.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .threads
    ///         .search(
    ///             &ThreadsSearchQueryRequest {
    ///                 q: Query("q".to_string()),
    ///                 limit: None,
    ///                 page_token: None,
    ///                 before: None,
    ///                 after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &ThreadsSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchThreadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/threads/search",
                None,
                QueryBuilder::new()
                    .serialize("q", Some(request.q.clone()))
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("before", request.before.clone())
                    .serialize("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail threads get --thread-id <thread_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .threads
    ///         .get(&ThreadID("thread_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        thread_id: &ThreadId,
        options: Option<RequestOptions>,
    ) -> Result<Thread, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/threads/{}", thread_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a thread and all of its messages.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail threads delete --thread-id <thread_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .threads
    ///         .delete(&ThreadID("thread_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        thread_id: &ThreadId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/threads/{}", thread_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .threads
    ///         .update(
    ///             &ThreadID("thread_id".to_string()),
    ///             &UpdateThreadRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        thread_id: &ThreadId,
        request: &UpdateThreadRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateThreadResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/threads/{}", thread_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail threads get-attachment --thread-id <thread_id> --attachment-id <attachment_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use agentmail_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = AgentmailClient::new(config).expect("Failed to build client");
    ///     client
    ///         .threads
    ///         .get_attachment(
    ///             &ThreadID("thread_id".to_string()),
    ///             &AttachmentID("attachment_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attachment(
        &self,
        thread_id: &ThreadId,
        attachment_id: &AttachmentId,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/threads/{}/attachments/{}", thread_id.0, attachment_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
