use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DraftsClient {
    pub http_client: HttpClient,
}

impl DraftsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail drafts list
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
    ///         .drafts
    ///         .list(
    ///             &DraftsListQueryRequest {
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
        request: &DraftsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDraftsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/drafts",
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

    /// **CLI:**
    /// ```bash
    /// agentmail drafts get --draft-id <draft_id>
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
    ///         .drafts
    ///         .get(&DraftID("draft_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        draft_id: &DraftId,
        options: Option<RequestOptions>,
    ) -> Result<Draft, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/drafts/{}", draft_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail drafts get-attachment --draft-id <draft_id> --attachment-id <attachment_id>
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
    ///         .drafts
    ///         .get_attachment(
    ///             &DraftID("draft_id".to_string()),
    ///             &AttachmentID("attachment_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attachment(
        &self,
        draft_id: &DraftId,
        attachment_id: &AttachmentId,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/drafts/{}/attachments/{}", draft_id.0, attachment_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
