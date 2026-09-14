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

    /// Lists every credential, newest first. Filter one family with `type`.
    /// Page to token exhaustion: a page can be empty and still carry a
    /// `next_page_token`.
    ///
    /// **CLI:**
    /// ```bash
    /// agentmail api-keys list
    /// ```
    ///
    /// # Arguments
    ///
    /// * `type_` - Restrict the list to one credential family. Omit for every family.
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
                    .serialize("type", request.r#type.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("ascending", request.ascending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a bearer key, or registers a public key when the body carries
    /// `public_key`. The route selects the scope. Bearer secrets are returned once.
    ///
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
    ///             &CreateAPIKeyRequest::CreateBearerAPIKeyRequest(CreateBearerAPIKeyRequest(
    ///                 APIKeyMutableFields {
    ///                     ..Default::default()
    ///                 },
    ///             )),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateApiKeyResult, ApiError> {
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

    /// Returns one credential of any family. Public keys also resolve by
    /// `client_id`. Poll a sign-in key until `status` is `active`.
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
    ///         .get(&APIKeyID("api_key_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        api_key_id: &ApiKeyId,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/api-keys/{}", api_key_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes one credential of any family. A pending sign-in key is
    /// cancelled; an active one is revoked. Public keys also resolve by `client_id`.
    ///
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

    /// Renames a credential or changes its permissions. Public keys also resolve
    /// by `client_id`; a sign-in key accepts only `provider_connect` and
    /// `provider_share_owner`.
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
    ///         .update(
    ///             &APIKeyID("api_key_id".to_string()),
    ///             &UpdateAPIKeyRequest(APIKeyMutableFields {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        api_key_id: &ApiKeyId,
        request: &UpdateApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/api-keys/{}", api_key_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
