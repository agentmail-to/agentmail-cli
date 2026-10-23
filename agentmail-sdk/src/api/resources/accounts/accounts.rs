use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AccountsClient {
    pub http_client: HttpClient,
}

impl AccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists accounts across all apps, scoped to the API key: an
    /// organization key sees every account, a pod key its pod's, an inbox key
    /// its inbox's. Requires `inbox_read`.
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
    ///         .accounts
    ///         .list(
    ///             &AccountsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v0/accounts",
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

    /// Returns one account by ID. An account outside the key's scope is a 404.
    /// Requires `inbox_read`.
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
    ///         .accounts
    ///         .get(&AccountID("account_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: &AccountId,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/accounts/{}", account_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates one account. Set `status` to `disabled` to stop the inbox from
    /// signing in at the app again, or to `enabled` to re-enable it.
    /// Idempotent: disabling an already disabled account keeps its original
    /// `disabled_at`, and enabling an enabled account is a no-op.
    ///
    /// Find the `account_id` with List Accounts. An account exists only after an
    /// inbox's first sign-in at an app, so it cannot be disabled in advance.
    /// A disable applies to that inbox at that app whichever sign-in key is
    /// used: the app's next authorization ends in `access_denied`, and a code
    /// issued earlier is refused with `invalid_grant`. Access tokens already
    /// issued stay valid until they expire, and the app's own session is
    /// unaffected.
    ///
    /// Requires `account_update`, which sign-in keys (`type: public_key`) cannot
    /// hold, so call this with a bearer API key. An account outside the key's
    /// scope is a 404. A 409 means the account changed during the write; read it
    /// again and retry.
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
    ///         .accounts
    ///         .update(
    ///             &AccountID("account_id".to_string()),
    ///             &UpdateAccountRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        account_id: &AccountId,
        request: &UpdateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v0/accounts/{}/update", account_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
