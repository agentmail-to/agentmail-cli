use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MessagesClient {
    pub http_client: HttpClient,
}

impl MessagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists messages in the inbox, most recent first. Pass `from`, `to`, or
    /// `subject` to filter by substring. Filtered requests are served by
    /// search, which caps `limit` at 100. For relevance-ranked full-text
    /// search across sender, recipients, subject, and message body, use
    /// `Search Messages`.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages list --inbox-id <inbox_id>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `from` - Filter to messages whose sender contains this value (substring match). Repeatable; all values must match.
    /// * `to` - Filter to messages whose recipients (to, cc, or bcc) contain this value (substring match). Repeatable; all values must match.
    /// * `subject` - Filter to messages whose subject contains this value (substring match). Repeatable; all values must match.
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
    ///         .inboxes
    ///         .messages
    ///         .list(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesMessagesListQueryRequest {
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
    ///                 from: None,
    ///                 to: None,
    ///                 subject: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesMessagesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/messages", inbox_id.0),
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
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .serialize("subject", request.subject.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Full-text search across messages in the inbox, ranked by relevance. The
    /// query is matched against the sender, recipients, and subject (substring)
    /// and the message body (tokenized full text). Spam, trash, blocked, and
    /// unauthenticated messages are always excluded. `limit` cannot exceed 100.
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
    ///         .inboxes
    ///         .messages
    ///         .search(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesMessagesSearchQueryRequest {
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
        inbox_id: &InboxesInboxId,
        request: &InboxesMessagesSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/messages/search", inbox_id.0),
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
    /// agentmail inboxes messages get --inbox-id <inbox_id> --message-id <message_id>
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
    ///         .inboxes
    ///         .messages
    ///         .get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        options: Option<RequestOptions>,
    ) -> Result<Message, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/messages/{}", inbox_id.0, message_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a message.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages delete --inbox-id <inbox_id> --message-id <message_id>
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
    ///         .inboxes
    ///         .messages
    ///         .delete(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}/messages/{}", inbox_id.0, message_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages update --inbox-id <inbox_id> --message-id <message_id> --add-labels read --remove-labels unread
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
    ///         .inboxes
    ///         .messages
    ///         .update(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             &UpdateMessageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        request: &UpdateMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}/messages/{}", inbox_id.0, message_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetch metadata for up to 500 messages in one request. Missing or
    /// restricted IDs are silently omitted; compare `count` against `limit`
    /// to detect misses.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages batch-get --inbox-id <inbox_id> --message-ids <id1> --message-ids <id2>
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
    ///         .inboxes
    ///         .messages
    ///         .batch_get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &BatchGetMessagesRequest {
    ///                 message_ids: BatchGetMessagesMessageIDs(vec![MessageID("message_ids".to_string())]),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_get(
        &self,
        inbox_id: &InboxesInboxId,
        request: &BatchGetMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchGetMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/messages/batch-get", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Apply one label change to up to 50 messages in a single request. The
    /// same add_labels and remove_labels apply to every message id, and at
    /// least one of them must be provided. The update is atomic: either all
    /// resolved messages are updated or none are. Missing or restricted ids
    /// are silently excluded; compare `count` against `limit` to detect
    /// exclusions.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages batch-update --inbox-id <inbox_id> --message-ids <id1> --message-ids <id2> --add-labels read --remove-labels unread
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
    ///         .inboxes
    ///         .messages
    ///         .batch_update(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &BatchUpdateMessagesRequest {
    ///                 message_ids: BatchUpdateMessagesMessageIDs(vec![MessageID(
    ///                     "message_ids".to_string(),
    ///                 )]),
    ///                 add_labels: None,
    ///                 remove_labels: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_update(
        &self,
        inbox_id: &InboxesInboxId,
        request: &BatchUpdateMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchUpdateMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/messages/batch-update", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages get-attachment --inbox-id <inbox_id> --message-id <message_id> --attachment-id <attachment_id>
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
    ///         .inboxes
    ///         .messages
    ///         .get_attachment(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             &AttachmentID("attachment_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attachment(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        attachment_id: &AttachmentId,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/inboxes/{}/messages/{}/attachments/{}",
                    inbox_id.0, message_id.0, attachment_id.0
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages get-raw --inbox-id <inbox_id> --message-id <message_id>
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
    ///         .inboxes
    ///         .messages
    ///         .get_raw(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_raw(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        options: Option<RequestOptions>,
    ) -> Result<RawMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/messages/{}/raw", inbox_id.0, message_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages send --inbox-id <inbox_id> --to recipient@example.com --subject "Hello" --text "Body"
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
    ///         .inboxes
    ///         .messages
    ///         .send(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &SendMessageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send(
        &self,
        inbox_id: &InboxesInboxId,
        request: &SendMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/messages/send", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages reply --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
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
    ///         .inboxes
    ///         .messages
    ///         .reply(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             &ReplyToMessageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reply(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        request: &ReplyToMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/messages/{}/reply", inbox_id.0, message_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages reply-all --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
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
    ///         .inboxes
    ///         .messages
    ///         .reply_all(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             &ReplyAllMessageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reply_all(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        request: &ReplyAllMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v0/inboxes/{}/messages/{}/reply-all",
                    inbox_id.0, message_id.0
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes messages forward --inbox-id <inbox_id> --message-id <message_id> --to recipient@example.com
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
    ///         .inboxes
    ///         .messages
    ///         .forward(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &MessageID("message_id".to_string()),
    ///             &SendMessageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn forward(
        &self,
        inbox_id: &InboxesInboxId,
        message_id: &MessageId,
        request: &SendMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v0/inboxes/{}/messages/{}/forward",
                    inbox_id.0, message_id.0
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
