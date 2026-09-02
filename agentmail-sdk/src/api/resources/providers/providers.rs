use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProvidersClient {
    pub http_client: HttpClient,
}

impl ProvidersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists providers, most popular first.
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
    ///         .providers
    ///         .list(
    ///             &ProvidersListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ProvidersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProvidersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/providers",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Searches providers by name prefix.
    ///
    /// # Arguments
    ///
    /// * `q` - Name prefix to search for.
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
    ///         .providers
    ///         .search(
    ///             &ProvidersSearchQueryRequest {
    ///                 q: "q".to_string(),
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &ProvidersSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchProvidersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/providers/search",
                None,
                QueryBuilder::new()
                    .string("q", request.q.clone())
                    .serialize("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

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
    ///         .providers
    ///         .get(&ProviderID("provider_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        provider_id: &ProviderId,
        options: Option<RequestOptions>,
    ) -> Result<Provider, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/providers/{}", provider_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists accounts at one provider, most recent sign-in first.
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
    ///         .providers
    ///         .list_accounts(
    ///             &ProviderID("provider_id".to_string()),
    ///             &ListAccountsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_accounts(
        &self,
        provider_id: &ProviderId,
        request: &ListAccountsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProviderAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/providers/{}/accounts", provider_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Starts signing an inbox in to a provider. Returns a `magic_url` valid
    /// for five minutes; open it in the browser that will hold the sign-in.
    /// Requires `api_key_create` and an `Idempotency-Key` header.
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
    ///         .providers
    ///         .connect(
    ///             &ProviderID("provider_id".to_string()),
    ///             &ConnectProviderBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connect(
        &self,
        provider_id: &ProviderId,
        request: &ConnectProviderBody,
        options: Option<RequestOptions>,
    ) -> Result<ConnectProviderAccepted, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/providers/{}/connect", provider_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
