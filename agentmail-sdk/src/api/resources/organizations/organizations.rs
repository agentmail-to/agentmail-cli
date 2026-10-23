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

    /// Returns whether the organization is in AgentMail Labs, which turns on features that are still in preview, such as [Agent Armor](https://docs.agentmail.to/agent-armor). Requires an organization-scoped API key.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail organizations get-labs
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
    ///     client.organizations.get_labs(None).await;
    /// }
    /// ```
    pub async fn get_labs(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationLabs, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v0/organizations/labs", None, None, options)
            .await
    }

    /// Joins AgentMail Labs for the whole organization, which turns on every feature in Labs for every inbox. Joining again has no effect. There is no leave on the API; an organization admin can leave in the console under [Settings, then Labs](https://console.agentmail.to/dashboard/settings/labs).
    ///
    /// Join only when your organization has decided to. Get Labs reads the same for an organization that never joined and one whose admin left Labs on purpose, so don't join just because `enabled` is `false`.
    ///
    /// Requires an organization-scoped API key with no permissions restriction, and a verified organization. API keys see the change within five minutes. Where Labs isn't offered (`available` is `false` on Get Labs), joining returns `403`.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail organizations join-labs
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
    ///     client.organizations.join_labs(None).await;
    /// }
    /// ```
    pub async fn join_labs(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationLabs, ApiError> {
        self.http_client
            .execute_request(Method::POST, "v0/organizations/labs", None, None, options)
            .await
    }
}
