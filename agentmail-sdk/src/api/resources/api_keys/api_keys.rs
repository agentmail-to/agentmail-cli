use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail api-keys list
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
    ///         .api_keys
    ///         .list(
    ///             &APIKeysListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ApiKeysListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListApiKeysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/api-keys",
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
    /// agentmail api-keys create --name "My Key"
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
    ///         .api_keys
    ///         .create(
    ///             &CreateAPIKeyRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateApiKeyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/api-keys",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail api-keys delete --api-key-id <api_key_id>
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
    ///         .api_keys
    ///         .delete(&APIKeyID("api_key_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        api_key_id: &ApiKeyId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/api-keys/{}", api_key_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// List only public-key credentials visible to the bearer caller's scope.
    /// Bearer credentials are never returned, even though both credential types
    /// share storage and pagination indexes. Requires `api_key_read`.
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
    ///         .api_keys
    ///         .list_public_keys(
    ///             &ListPublicKeysQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_public_keys(
        &self,
        request: &ListPublicKeysQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPublicKeysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/api-keys/public-keys",
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

    /// Register a public P-256 JWK using an existing AgentMail bearer API key
    /// with `api_key_create`. Re-registering the same JWK creates a new
    /// credential ID; it does not replace or recover an earlier credential.
    /// The private key must never be sent to AgentMail.
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
    ///         .api_keys
    ///         .create_public_key(
    ///             &CreatePublicKeyRequest {
    ///                 public_key: PublicJwk {
    ///                     kty: PublicJwkKty::Ec,
    ///                     crv: PublicJwkCrv::P256,
    ///                     x: PublicJwkCoordinate("x".to_string()),
    ///                     y: PublicJwkCoordinate("y".to_string()),
    ///                 },
    ///                 name: None,
    ///                 scope: None,
    ///                 expires_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_public_key(
        &self,
        request: &CreatePublicKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<PublicKeyCredential, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/api-keys/public-keys",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Permanently revoke one public-key credential. This hard-deletes the
    /// credential; repeating the request returns not found. Requires
    /// `api_key_delete`.
    ///
    /// # Arguments
    ///
    /// * `api_key_id` - Public-key credential ID returned by registration.
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
    ///         .api_keys
    ///         .revoke_public_key(&"api_key_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn revoke_public_key(
        &self,
        api_key_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/api-keys/public-keys/{}", api_key_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Rename the credential. All security-relevant fields are immutable.
    /// Requires `api_key_update`.
    ///
    /// # Arguments
    ///
    /// * `api_key_id` - Public-key credential ID returned by registration.
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
    ///         .api_keys
    ///         .update_public_key_name(
    ///             &"api_key_id".to_string(),
    ///             &UpdatePublicKeyNameRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_public_key_name(
        &self,
        api_key_id: &str,
        request: &UpdatePublicKeyNameRequest,
        options: Option<RequestOptions>,
    ) -> Result<PublicKeyCredential, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/api-keys/public-keys/{}", api_key_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Invalidate every current public-key credential in the caller's
    /// organization by advancing its AgentID key generation. The caller must be
    /// organization-scoped and either have `api_key_delete` or, for a verified
    /// self-serve agent organization, use an unrestricted unmanaged bearer
    /// credential. No request body is accepted.
    ///
    /// `Idempotency-Key` is required and must be a UUID. Reusing the same UUID
    /// returns the original permanent receipt without advancing the generation
    /// again. A new UUID performs a new generation advance.
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
    ///         .api_keys
    ///         .revoke_all_agent_id_sign_in_keys(Some(
    ///             RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn revoke_all_agent_id_sign_in_keys(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<RevokeAllAgentIdSignInKeysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v0/api-keys/public-keys/agentid-sign-in/revoke-all",
                None,
                None,
                options,
            )
            .await
    }

    /// List active browser credentials visible to the caller's scope. Requires `api_key_read`.
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
    ///         .api_keys
    ///         .list_browser_credentials(
    ///             &ListBrowserCredentialsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_browser_credentials(
        &self,
        request: &ListBrowserCredentialsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBrowserCredentialsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/api-keys/browser-credentials",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List owner-facing browser credential and consent lifecycle events. Requires `api_key_read`.
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
    ///         .api_keys
    ///         .list_browser_credential_events(
    ///             &ListBrowserCredentialEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_browser_credential_events(
        &self,
        request: &ListBrowserCredentialEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBrowserLifecycleEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/api-keys/browser-credentials/events",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Permanently revoke one active browser credential. Requires `api_key_delete`.
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
    ///         .api_keys
    ///         .delete_browser_credential(&"credential_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_browser_credential(
        &self,
        credential_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/api-keys/browser-credentials/{}", credential_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel one pending, unexpired browser enrollment intent. Requires `api_key_delete`.
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
    ///         .api_keys
    ///         .cancel_browser_enrollment(&"enrollment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_browser_enrollment(
        &self,
        enrollment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v0/api-keys/browser-credentials/enrollments/{}",
                    enrollment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// List remembered AgentID client approvals for one live inbox. Requires `api_key_read`.
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
    ///         .api_keys
    ///         .list_browser_consents(
    ///             &ListBrowserConsentsQueryRequest {
    ///                 inbox_id: "inbox_id".to_string(),
    ///                 limit: None,
    ///                 page_token: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_browser_consents(
        &self,
        request: &ListBrowserConsentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBrowserConsentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/api-keys/browser-consents",
                None,
                QueryBuilder::new()
                    .string("inbox_id", request.inbox_id.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Revoke one remembered AgentID client approval. Requires `api_key_delete`.
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
    ///         .api_keys
    ///         .delete_browser_consent(&"consent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_browser_consent(
        &self,
        consent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/api-keys/browser-consents/{}", consent_id),
                None,
                None,
                options,
            )
            .await
    }
}
