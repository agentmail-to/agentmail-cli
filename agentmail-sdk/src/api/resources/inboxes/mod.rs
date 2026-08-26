use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod api_keys;
pub use api_keys::ApiKeysClient2;
pub mod drafts;
pub use drafts::DraftsClient2;
pub mod events;
pub use events::EventsClient;
pub mod lists;
pub use lists::ListsClient2;
pub mod messages;
pub use messages::MessagesClient;
pub mod metrics;
pub use metrics::MetricsClient2;
pub mod threads;
pub use threads::ThreadsClient2;
pub mod webhooks;
pub use webhooks::WebhooksClient2;
pub struct InboxesClient {
    pub http_client: HttpClient,
    pub api_keys: ApiKeysClient2,
    pub drafts: DraftsClient2,
    pub events: EventsClient,
    pub lists: ListsClient2,
    pub messages: MessagesClient,
    pub metrics: MetricsClient2,
    pub threads: ThreadsClient2,
    pub webhooks: WebhooksClient2,
}

impl InboxesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            api_keys: ApiKeysClient2::new(config.clone())?,
            drafts: DraftsClient2::new(config.clone())?,
            events: EventsClient::new(config.clone())?,
            lists: ListsClient2::new(config.clone())?,
            messages: MessagesClient::new(config.clone())?,
            metrics: MetricsClient2::new(config.clone())?,
            threads: ThreadsClient2::new(config.clone())?,
            webhooks: WebhooksClient2::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes list
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
    ///         .list(
    ///             &InboxesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &InboxesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboxesListInboxesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/inboxes",
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
    /// agentmail inboxes create --display-name "My Agent" --username myagent --domain agentmail.to
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
    ///         .create(
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
        request: &InboxesCreateInboxRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboxesInbox, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/inboxes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes get --inbox-id <inbox_id>
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
    ///         .get(&InboxesInboxID("inbox_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        options: Option<RequestOptions>,
    ) -> Result<InboxesInbox, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}", inbox_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes delete --inbox-id <inbox_id>
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
    ///         .delete(&InboxesInboxID("inbox_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}", inbox_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes update --inbox-id <inbox_id> --display-name "Updated Name"
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
    ///         .update(
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
        inbox_id: &InboxesInboxId,
        request: &InboxesUpdateInboxRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboxesInbox, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/inboxes/{}", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
