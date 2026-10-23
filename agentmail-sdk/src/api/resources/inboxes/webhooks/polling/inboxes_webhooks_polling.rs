use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PollingClient {
    pub http_client: HttpClient,
}

impl PollingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the polling webhooks created through this inbox route.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks polling list --inbox-id <inbox_id>
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
    ///         .webhooks
    ///         .polling
    ///         .list(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesWebhooksPollingListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesWebhooksPollingListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListPollingWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/webhooks/polling", inbox_id.0),
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

    /// Create a polling webhook scoped to this inbox and return the `token` that reads it.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks polling create --inbox-id <inbox_id> --client-id my-agent --event-types message.received
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
    ///         .webhooks
    ///         .polling
    ///         .create(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksCreateInboxPollingWebhookRequest {
    ///                 client_id: WebhooksPollingClientID(WebhooksClientID("client_id".to_string())),
    ///                 event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
    ///                     EventType::MessageReceived,
    ///                 ])),
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
        request: &WebhooksCreateInboxPollingWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksCreatePollingWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/webhooks/polling", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks polling get --inbox-id <inbox_id> --webhook-id <webhook_id>
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
    ///         .webhooks
    ///         .polling
    ///         .get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksPollingWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v0/inboxes/{}/webhooks/polling/{}",
                    inbox_id.0, webhook_id.0
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail inboxes webhooks polling delete --inbox-id <inbox_id> --webhook-id <webhook_id>
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
    ///         .webhooks
    ///         .polling
    ///         .delete(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &WebhooksWebhookID("webhook_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        webhook_id: &WebhooksWebhookId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v0/inboxes/{}/webhooks/polling/{}",
                    inbox_id.0, webhook_id.0
                ),
                None,
                None,
                options,
            )
            .await
    }
}
