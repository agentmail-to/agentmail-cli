use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod api_keys;
pub use api_keys::ApiKeysClient3;
pub mod domains;
pub use domains::DomainsClient2;
pub mod drafts;
pub use drafts::DraftsClient3;
pub mod inboxes;
pub use inboxes::InboxesClient2;
pub mod lists;
pub use lists::ListsClient3;
pub mod metrics;
pub use metrics::MetricsClient3;
pub mod threads;
pub use threads::ThreadsClient3;
pub mod webhooks;
pub use webhooks::WebhooksClient3;
pub struct PodsClient {
    pub http_client: HttpClient,
    pub api_keys: ApiKeysClient3,
    pub domains: DomainsClient2,
    pub drafts: DraftsClient3,
    pub inboxes: InboxesClient2,
    pub lists: ListsClient3,
    pub metrics: MetricsClient3,
    pub threads: ThreadsClient3,
    pub webhooks: WebhooksClient3,
}

impl PodsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            api_keys: ApiKeysClient3::new(config.clone())?,
            domains: DomainsClient2::new(config.clone())?,
            drafts: DraftsClient3::new(config.clone())?,
            inboxes: InboxesClient2::new(config.clone())?,
            lists: ListsClient3::new(config.clone())?,
            metrics: MetricsClient3::new(config.clone())?,
            threads: ThreadsClient3::new(config.clone())?,
            webhooks: WebhooksClient3::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods list
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
    ///         .list(
    ///             &PodsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PodsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PodsListPodsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/pods",
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
    /// agentmail pods create --client-id my-pod
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
    ///         .create(
    ///             &PodsCreatePodRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &PodsCreatePodRequest,
        options: Option<RequestOptions>,
    ) -> Result<PodsPod, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/pods",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods get --pod-id <pod_id>
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
    ///         .get(&PodsPodID("pod_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        pod_id: &PodsPodId,
        options: Option<RequestOptions>,
    ) -> Result<PodsPod, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}", pod_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail pods delete --pod-id <pod_id>
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
    ///         .delete(&PodsPodID("pod_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        pod_id: &PodsPodId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/pods/{}", pod_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
