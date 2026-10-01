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
    /// `human_email` is optional. Without it, the inbox can receive email but cannot send to anyone until a human is attached with the attach human endpoint. There is also no way to recover the API key, so store it durably. Calling sign-up again without `human_email` creates a new organization, which needs a different `username`: the original username stays with the lost organization's inbox.
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
    ///                 username: "username".to_string(),
    ///                 human_email: None,
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

    /// Attach a human to an unverified agent organization. A 6-digit OTP is sent to the human's email, which you then submit to the verify endpoint.
    ///
    /// Use it after signing up without a `human_email`. Once the human is attached, the organization can send email to that human only, and verification lifts the remaining restrictions. For up to 5 minutes after attaching, sends to the human can still be rejected with a `429` daily send limit error while the API key's cached limits catch up. Wait and retry.
    ///
    /// Calling it again with the same `human_email` does not rotate the API key. It resends the OTP if it was never delivered, or issues a new one if it expired. While the current OTP is still valid, calling it again keeps that OTP and its attempt count. If all 10 attempts are used up, wait until the OTP expires, 24 hours after it was issued, then call it again for a new one.
    ///
    /// Calling it with a different `human_email` replaces the attached human and sends the new human an OTP. An organization can replace its human at most 2 times.
    ///
    /// Only available until the organization is verified.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail agent attach-human --human-email user@example.com
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
    ///         .attach_human(
    ///             &AgentAttachHumanRequest {
    ///                 human_email: "human_email".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn attach_human(
        &self,
        request: &AgentAttachHumanRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentAttachHumanResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/agent/human",
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
    /// The OTP expires after 24 hours and allows a maximum of 10 attempts. If the OTP expired, call the attach human endpoint with the same `human_email` to get a new one without rotating the API key. Once all 10 attempts are used, even the correct OTP is rejected, and attach human keeps returning the same OTP until it expires, so wait for it to expire before asking for a new one. An organization that signed up without a `human_email` has no OTP until a human is attached. If you run into any difficulties receiving the OTP code, you can also create an account on [console.agentmail.to](https://console.agentmail.to) using the human email address you provided to verify your account.
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
