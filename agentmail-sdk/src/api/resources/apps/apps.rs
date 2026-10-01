use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AppsClient {
    pub http_client: HttpClient,
}

impl AppsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists apps, most popular first.
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
    ///         .apps
    ///         .list(
    ///             &AppsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AppsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAppsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/apps",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Searches apps by name prefix.
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
    ///         .apps
    ///         .search(
    ///             &AppsSearchQueryRequest {
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
        request: &AppsSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchAppsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/apps/search",
                None,
                QueryBuilder::new()
                    .string("q", request.q.clone())
                    .serialize("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Gets one app by ID. An app in the catalog returns its full entry.
    /// A registered app that the catalog does not list returns its ID and
    /// name only, without `updated_at`, so anyone holding its ID can still look
    /// it up. List Apps and Search Apps show catalog entries only.
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
    ///     client.apps.get(&AppID("app_id".to_string()), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        app_id: &AppId,
        options: Option<RequestOptions>,
    ) -> Result<App, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/apps/{}", app_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists accounts at one app, most recent sign-in first.
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
    ///         .apps
    ///         .list_accounts(
    ///             &AppID("app_id".to_string()),
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
        app_id: &AppId,
        request: &ListAccountsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAppAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/apps/{}/accounts", app_id.0),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Starts signing an inbox in to an app. Returns a single-use `magic_url`,
    /// valid for five minutes, to open in the client that will hold the sign-in;
    /// the client enrolls as the inbox and continues to the app.
    /// A `404` names the missing resource: `App` or `Inbox`.
    /// A `403` `AppSignupLimitError` means the app accepts no more sign-ups from
    /// your organization; sign in with an inbox that already has an account there.
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
    ///         .apps
    ///         .connect(
    ///             &AppID("app_id".to_string()),
    ///             &ConnectAppBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connect(
        &self,
        app_id: &AppId,
        request: &ConnectAppBody,
        options: Option<RequestOptions>,
    ) -> Result<ConnectAccepted, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/apps/{}/connect", app_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
