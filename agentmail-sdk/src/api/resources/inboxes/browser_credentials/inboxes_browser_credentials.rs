use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BrowserCredentialsClient {
    pub http_client: HttpClient,
}

impl BrowserCredentialsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Attach a browser enrollment intent to the inbox. Requires
    /// `api_key_create`. Before submitting `transaction_jti`, independently
    /// verify that the browser page's final origin is exactly
    /// `https://auth.agentid.com`.
    ///
    /// This endpoint is available to every organization using US production.
    /// It is not available in EU production.
    ///
    /// Select `inbox_id` from trusted AgentMail configuration. An AgentID
    /// `login_hint` is not authoritative for selecting the inbox; when the
    /// transaction includes one, it must match the path inbox.
    ///
    /// **AgentMail API keys are sent only to `https://api.agentmail.to`; AgentID never requests them.**
    ///
    /// A new intent returns `202`; an idempotent retry for the same pending
    /// transaction, inbox, and bearer key returns `200` with the same receipt.
    /// An intent lasts at most five minutes. An activated credential lasts at
    /// most 30 days and cannot outlive its authorizing bearer API key.
    ///
    /// Creation is limited to 20 intents per bearer API key per hour, 100 per
    /// organization per hour, and five live unused intents per bearer API key.
    /// Browser activation is separately limited to 20 activations per
    /// authorizing bearer API key per UTC day. Either kind of limit can return
    /// `429`; honor the `Retry-After` header. Cancelling an enrollment releases
    /// its live-intent slot but does not reset the daily activation counter.
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
    ///         .browser_credentials
    ///         .create_enrollment(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CreateBrowserEnrollmentRequest {
    ///                 transaction_jti: BrowserEnrollmentTransactionJti("transaction_jti".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_enrollment(
        &self,
        inbox_id: &InboxesInboxId,
        request: &CreateBrowserEnrollmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<BrowserEnrollmentAccepted, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/browser-credentials/enrollments", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
