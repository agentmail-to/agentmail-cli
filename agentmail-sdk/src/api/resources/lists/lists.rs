use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ListsClient {
    pub http_client: HttpClient,
}

impl ListsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists list --direction <direction> --type <type>
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
    ///         .lists
    ///         .list(
    ///             &Direction::Send,
    ///             &ListType::Allow,
    ///             &ListsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        direction: &Direction,
        type_: &ListType,
        request: &ListsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListListEntriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/lists/{}/{}", direction, type_),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists create --direction <direction> --type <type> --entry user@example.com
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
    ///         .lists
    ///         .create(
    ///             &Direction::Send,
    ///             &ListType::Allow,
    ///             &CreateListEntryRequest {
    ///                 entry: "entry".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        direction: &Direction,
        type_: &ListType,
        request: &CreateListEntryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEntry, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/lists/{}/{}", direction, type_),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists get --direction <direction> --type <type> --entry <entry>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `entry` - Email address or domain.
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
    ///         .lists
    ///         .get(
    ///             &Direction::Send,
    ///             &ListType::Allow,
    ///             &"entry".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        direction: &Direction,
        type_: &ListType,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListEntry, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/lists/{}/{}/{}", direction, type_, entry),
                None,
                None,
                options,
            )
            .await
    }

    /// **CLI:**
    /// ```bash
    /// agentmail lists delete --direction <direction> --type <type> --entry <entry>
    /// ```
    ///
    /// # Arguments
    ///
    /// * `entry` - Email address or domain.
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
    ///         .lists
    ///         .delete(
    ///             &Direction::Send,
    ///             &ListType::Allow,
    ///             &"entry".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        direction: &Direction,
        type_: &ListType,
        entry: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/lists/{}/{}/{}", direction, type_, entry),
                None,
                None,
                options,
            )
            .await
    }
}
