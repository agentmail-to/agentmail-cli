use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DraftsClient3 {
    pub http_client: HttpClient,
}

impl DraftsClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods drafts list --pod-id <pod_id>
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
    ///         .pods
    ///         .drafts
    ///         .list(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsDraftsListQueryRequest {
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
        pod_id: &PodsPodId,
        request: &PodsDraftsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDraftsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/drafts", pod_id.0),
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
    /// agentmail pods drafts get --pod-id <pod_id> --draft-id <draft_id>
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
    ///         .pods
    ///         .drafts
    ///         .get(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        pod_id: &PodsPodId,
        draft_id: &DraftId,
        options: Option<RequestOptions>,
    ) -> Result<Draft, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/drafts/{}", pod_id.0, draft_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods drafts get-attachment --pod-id <pod_id> --draft-id <draft_id> --attachment-id <attachment_id>
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
    ///         .pods
    ///         .drafts
    ///         .get_attachment(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &DraftID("draft_id".to_string()),
    ///             &AttachmentID("attachment_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attachment(
        &self,
        pod_id: &PodsPodId,
        draft_id: &DraftId,
        attachment_id: &AttachmentId,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/pods/{}/drafts/{}/attachments/{}",
                    pod_id.0, draft_id.0, attachment_id.0
                ),
                None,
                None,
                options,
            )
            .await
    }
}
