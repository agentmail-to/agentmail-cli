use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OrganizationsClient {
    pub http_client: HttpClient,
}

impl OrganizationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the organization for the authenticated API key (usage limits, counts, and billing metadata).
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail organizations get
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
    ///     client.organizations.get(None).await;
    /// }
    /// ```
    pub async fn get(&self, options: Option<RequestOptions>) -> Result<Organization, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v0/organizations", None, None, options)
            .await
    }
}
