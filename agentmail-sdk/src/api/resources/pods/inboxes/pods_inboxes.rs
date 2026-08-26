use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InboxesClient2 {
    pub http_client: HttpClient,
}

impl InboxesClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods inboxes list --pod-id <pod_id>
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
    ///         .inboxes
    ///         .list(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsInboxesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        pod_id: &PodsPodId,
        request: &PodsInboxesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboxesListInboxesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/inboxes", pod_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("ascending", request.ascending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods inboxes create --pod-id <pod_id> --username myagent --domain example.com
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
    ///         .inboxes
    ///         .create(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &InboxesCreateInboxRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        pod_id: &PodsPodId,
        request: &InboxesCreateInboxRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboxesInbox, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/pods/{}/inboxes", pod_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods inboxes get --pod-id <pod_id> --inbox-id <inbox_id>
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
    ///         .inboxes
    ///         .get(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        pod_id: &PodsPodId,
        inbox_id: &InboxesInboxId,
        options: Option<RequestOptions>,
    ) -> Result<InboxesInbox, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/inboxes/{}", pod_id.0, inbox_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods inboxes delete --pod-id <pod_id> --inbox-id <inbox_id>
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
    ///         .pods
    ///         .inboxes
    ///         .delete(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        pod_id: &PodsPodId,
        inbox_id: &InboxesInboxId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/pods/{}/inboxes/{}", pod_id.0, inbox_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods inboxes update --pod-id <pod_id> --inbox-id <inbox_id>
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
    ///         .inboxes
    ///         .update(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesUpdateInboxRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        pod_id: &PodsPodId,
        inbox_id: &InboxesInboxId,
        request: &InboxesUpdateInboxRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboxesInbox, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/pods/{}/inboxes/{}", pod_id.0, inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
