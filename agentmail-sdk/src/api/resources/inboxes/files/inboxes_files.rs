use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FilesClient {
    pub http_client: HttpClient,
}

impl FilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the inbox's files in path order, pending ones included, optionally only those whose path starts with
    /// `path_prefix`. A prefix matches characters, not whole segments: `reports/q` matches `reports/q3.pdf`. Items
    /// carry no download URL; get a file for one.
    ///
    /// The list is read in the region that serves the request, so a file created or deleted a moment ago may not
    /// show yet. Expired files never show.
    ///
    /// Requires the `file_read` permission.
    ///
    /// # Arguments
    ///
    /// * `path_prefix` - Only files whose path starts with this. Held to the path rules, except it may stop partway.
    /// * `limit` - Page size, 100 by default and at most 500.
    /// * `ascending` - Path order, `true` by default. `false` lists in reverse.
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
    ///         .inboxes
    ///         .files
    ///         .list(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &InboxesFilesListQueryRequest {
    ///                 path_prefix: Some("reports/".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        inbox_id: &InboxesInboxId,
        request: &InboxesFilesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/files", inbox_id.0),
                None,
                QueryBuilder::new()
                    .serialize("path_prefix", request.path_prefix.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("ascending", request.ascending.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Starts an upload: creates a pending file at `path` and returns an `upload` to `PUT` its bytes to. The bytes go
    /// straight to storage, not through the API. Then call Complete File to make the file `ready`.
    ///
    /// Retry-safe: sending the identical request again (same `path`, `size`, `sha256`, `content_type` and
    /// `expires_at`) returns the existing file with `200` instead of `201`: a pending one with a fresh `upload`, or
    /// a `ready` one without. If its bytes were uploaded but the upload was never completed, it is completed then. A
    /// different request at a path whose file is still pending and has no bytes replaces that file; at any other
    /// taken path it returns `409`.
    ///
    /// The file counts against the organization's Drive quota from creation, pending or not: 1 GiB and 10,000 files
    /// by default. A file that doesn't fit returns `403` with code `limit_exceeded`. Deleting a file frees its
    /// space within seconds. Expired files count until a create needs their space: a create that doesn't fit first
    /// gives back what expired files hold, a batch at a time, so retry a refusal while many expired files remain.
    ///
    /// Requires the `file_create` permission, which an unverified organization's keys don't have at any scope until
    /// it completes agent verification. A key without `file_read` can still create files, but gets the plain `409` at
    /// any taken path, even for an identical request at a file it didn't create.
    ///
    /// Agent Drive is in AgentMail Labs, available in US production (`api.agentmail.to`) but not yet in EU
    /// production (`api.agentmail.eu`). Organizations that have not joined Labs receive a `403`.
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
    ///         .inboxes
    ///         .files
    ///         .create(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &CreateFileRequest {
    ///                 path: FilePath("reports/q3.pdf".to_string()),
    ///                 size: FileSize(48213),
    ///                 sha256: FileSha256("n4bQgYhMfWWaL+qgxVrQFaO/TxsrC4Is0V1sFbDwCgg=".to_string()),
    ///                 content_type: Some(FileContentType("application/pdf".to_string())),
    ///                 expires_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        inbox_id: &InboxesInboxId,
        request: &CreateFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateFileResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/files", inbox_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Makes a pending file `ready` once its bytes are uploaded, and returns it, with its `download_url` if the key
    /// has `file_read`. Calling it on a file that is already `ready` returns the file.
    ///
    /// Returns `409` if the bytes haven't been uploaded yet, or if what was uploaded doesn't match the file's `size`
    /// and `sha256`. Returns `404` for a file that has expired or was deleted, including a pending file not
    /// completed within a day of its creation.
    ///
    /// Requires the `file_create` permission. Only a key with `file_read`, or the key that created the file, can
    /// complete it; any other key gets `404`.
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
    ///         .inboxes
    ///         .files
    ///         .complete(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &FileID("file_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete(
        &self,
        inbox_id: &InboxesInboxId,
        file_id: &FileId,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v0/inboxes/{}/files/{}/complete", inbox_id.0, file_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets a file. A `ready` file comes with a fresh `download_url`, valid for up to an hour.
    ///
    /// A get reads the region that serves the request, so for about a second after a delete it can still return
    /// the deleted file. A file that has expired returns `404` at once.
    ///
    /// Requires the `file_read` permission.
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
    ///         .inboxes
    ///         .files
    ///         .get(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &FileID("file_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        inbox_id: &InboxesInboxId,
        file_id: &FileId,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v0/inboxes/{}/files/{}", inbox_id.0, file_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a file, pending or `ready`, and frees its path at once. Its bytes are deleted, and its space given
    /// back to the organization's quota, within seconds. A download URL signed before the delete keeps working
    /// until it expires, at most an hour. Deleting a file that is already gone returns `404`.
    ///
    /// Requires the `file_delete` permission. Only a key with `file_read`, or the key that created the file, can
    /// delete it; any other key gets `404`.
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
    ///         .inboxes
    ///         .files
    ///         .delete(
    ///             &InboxesInboxID("inbox_id".to_string()),
    ///             &FileID("file_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        inbox_id: &InboxesInboxId,
        file_id: &FileId,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v0/inboxes/{}/files/{}", inbox_id.0, file_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
