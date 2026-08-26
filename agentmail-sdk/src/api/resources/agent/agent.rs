use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AgentClient {
    pub http_client: HttpClient,
}

impl AgentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new agent organization with an inbox and API key. This endpoint is for signing up for the first time. If you've already signed up, you're all set — just use your existing API key.
    ///
    /// A 6-digit OTP is sent to the human's email for verification.
    ///
    /// This endpoint is idempotent. Calling it again with the same `human_email` will rotate the API key and resend the OTP if expired.
    ///
    /// The returned API key has limited permissions until the organization is verified via the verify endpoint.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail agent sign-up --human-email user@example.com --username my-agent
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
    ///         .agent
    ///         .sign_up(
    ///             &AgentSignupRequest {
    ///                 human_email: "human_email".to_string(),
    ///                 username: "username".to_string(),
    ///                 source: None,
    ///                 referrer: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn sign_up(
        &self,
        request: &AgentSignupRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentSignupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/agent/sign-up",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Verify an agent organization using the 6-digit OTP sent to the human's email during sign-up.
    ///
    /// On success, the organization is upgraded from `agent_unverified` to `agent_verified`, the send allowlist is removed, and free plan entitlements are applied.
    ///
    /// The OTP expires after 24 hours and allows a maximum of 10 attempts. If you run into any difficulties receiving the OTP code, you can also create an account on [console.agentmail.to](https://console.agentmail.to) using the human email address you provided to verify your account.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail agent verify --otp-code 123456
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
    ///         .agent
    ///         .verify(
    ///             &AgentVerifyRequest {
    ///                 otp_code: "otp_code".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn verify(
        &self,
        request: &AgentVerifyRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentVerifyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/agent/verify",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
