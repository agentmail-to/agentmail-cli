use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DraftsClient2 {
    pub http_client: HttpClient,
}

impl DraftsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts list --inbox-id <inbox_id>
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
    ///         .drafts
    ///         .list(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesDraftsListQueryRequest {
    ///                 limit: None,
    ///                 page_token: None,
    ///                 labels: vec![],
    ///                 before: None,
    ///                 after: None,
    ///                 ascending: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesDraftsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDraftsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/drafts", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .string_array("labels", request.labels.clone())
                    .serialize("before", request.before.clone())
                    .serialize("after", request.after.clone())
                    .serialize("ascending", request.ascending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a draft. Supply `in_reply_to` to create a reply draft (with
    /// `reply_all` to address the whole thread), whose recipients, subject, and
    /// threading are derived from the referenced message, or `forward_of` to
    /// create a forward draft, which derives the subject, threading, and
    /// forwarded content from the source but keeps recipients caller-supplied.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts create --inbox-id <inbox_id> --to recipient@example.com --subject "Draft subject" --text "Draft body"
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
    ///         .drafts
    ///         .create(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CreateDraftRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        inbox_id: &InboxesInboxId,
        request: &CreateDraftRequest,
        options: Option<RequestOptions>,
    ) -> Result<Draft, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/drafts", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts get --inbox-id <inbox_id> --draft-id <draft_id>
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
    ///         .drafts
    ///         .get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        draft_id: &DraftId,
        options: Option<RequestOptions>,
    ) -> Result<Draft, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/drafts/{}", inbox_id.0, draft_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts delete --inbox-id <inbox_id> --draft-id <draft_id>
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
    ///         .drafts
    ///         .delete(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        draft_id: &DraftId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}/drafts/{}", inbox_id.0, draft_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit fields on an existing draft. Passing `null` clears a field (or `[]`
    /// for a recipient field); `send_at: null` un-schedules a scheduled draft.
    /// A draft that is already being sent cannot be edited.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts update --inbox-id <inbox_id> --draft-id <draft_id> --subject "Updated subject"
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
    ///         .drafts
    ///         .update(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             &UpdateDraftRequest {
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
        draft_id: &DraftId,
        request: &UpdateDraftRequest,
        options: Option<RequestOptions>,
    ) -> Result<Draft, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}/drafts/{}", inbox_id.0, draft_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts get-attachment --inbox-id <inbox_id> --draft-id <draft_id> --attachment-id <attachment_id>
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
    ///         .drafts
    ///         .get_attachment(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             &AttachmentID("attachment_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attachment(
        &self,
        inbox_id: &InboxesInboxId,
        draft_id: &DraftId,
        attachment_id: &AttachmentId,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/inboxes/{}/drafts/{}/attachments/{}",
                    inbox_id.0, draft_id.0, attachment_id.0
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes drafts send --inbox-id <inbox_id> --draft-id <draft_id>
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
    ///         .drafts
    ///         .send(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             &UpdateMessageRequest {
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
        draft_id: &DraftId,
        request: &UpdateMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/drafts/{}/send", inbox_id.0, draft_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
