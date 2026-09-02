//! API client and types for the AgentMail
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AccountsClient, AgentClient, ApiClient, ApiKeysClient, AuthClient, DomainsClient, DraftsClient,
    InboxesClient, ListsClient, MetricsClient, OrganizationsClient, PodsClient, ProvidersClient,
    ThreadsClient, WebhooksClient,
};

pub use agentmail_types::*;
