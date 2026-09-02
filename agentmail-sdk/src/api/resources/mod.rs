//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Inboxes**
//! - **Pods**
//! - **Webhooks**
//! - **Accounts**
//! - **Agent**
//! - **ApiKeys**
//! - **Auth**
//! - **Domains**
//! - **Drafts**
//! - **Lists**
//! - **Metrics**
//! - **Organizations**
//! - **Providers**
//! - **Threads**

use crate::{ApiError, ClientConfig};

pub mod accounts;
pub mod agent;
pub mod api_keys;
pub mod auth;
pub mod domains;
pub mod drafts;
pub mod inboxes;
pub mod lists;
pub mod metrics;
pub mod organizations;
pub mod pods;
pub mod providers;
pub mod threads;
pub mod webhooks;
pub struct ApiClient {
    pub config: ClientConfig,
    pub inboxes: InboxesClient,
    pub pods: PodsClient,
    pub webhooks: WebhooksClient,
    pub accounts: AccountsClient,
    pub agent: AgentClient,
    pub api_keys: ApiKeysClient,
    pub auth: AuthClient,
    pub domains: DomainsClient,
    pub drafts: DraftsClient,
    pub lists: ListsClient,
    pub metrics: MetricsClient,
    pub organizations: OrganizationsClient,
    pub providers: ProvidersClient,
    pub threads: ThreadsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            inboxes: InboxesClient::new(config.clone())?,
            pods: PodsClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            accounts: AccountsClient::new(config.clone())?,
            agent: AgentClient::new(config.clone())?,
            api_keys: ApiKeysClient::new(config.clone())?,
            auth: AuthClient::new(config.clone())?,
            domains: DomainsClient::new(config.clone())?,
            drafts: DraftsClient::new(config.clone())?,
            lists: ListsClient::new(config.clone())?,
            metrics: MetricsClient::new(config.clone())?,
            organizations: OrganizationsClient::new(config.clone())?,
            providers: ProvidersClient::new(config.clone())?,
            threads: ThreadsClient::new(config.clone())?,
        })
    }
}

pub use accounts::AccountsClient;
pub use agent::AgentClient;
pub use api_keys::ApiKeysClient;
pub use auth::AuthClient;
pub use domains::DomainsClient;
pub use drafts::DraftsClient;
pub use inboxes::InboxesClient;
pub use lists::ListsClient;
pub use metrics::MetricsClient;
pub use organizations::OrganizationsClient;
pub use pods::PodsClient;
pub use providers::ProvidersClient;
pub use threads::ThreadsClient;
pub use webhooks::WebhooksClient;
