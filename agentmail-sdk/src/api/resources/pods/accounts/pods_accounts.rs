use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AccountsClient3 {
    pub http_client: HttpClient,
}

impl AccountsClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists accounts held by inboxes in the pod, across all providers. Requires `inbox_read`.
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
    ///         .pods
    ///         .accounts
    ///         .list(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &PodsAccountsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        pod_id: &PodsPodId,
        request: &PodsAccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/accounts", pod_id.0),
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

    /// Returns one account held by inboxes in the pod. An account elsewhere is a 404. Requires
    /// `inbox_read`.
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
    ///         .pods
    ///         .accounts
    ///         .get(
    ///             &PodsPodID("pod_id".to_string()),
    ///             &AccountID("account_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        pod_id: &PodsPodId,
        account_id: &AccountId,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/pods/{}/accounts/{}", pod_id.0, account_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
