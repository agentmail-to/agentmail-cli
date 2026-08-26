use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AuthClient {
    pub http_client: HttpClient,
}

impl AuthClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the identity and scope of the authenticated credential. Useful when a client holds a pod-scoped or inbox-scoped API key and needs to discover the parent organization, pod, or inbox without prior knowledge.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail auth me
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
    ///     client.auth.me(None).await;
    /// }
    /// ```
    pub async fn me(&self, options: Option<RequestOptions>) -> Result<Identity, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v0/auth/me", None, None, options)
            .await
    }
}
