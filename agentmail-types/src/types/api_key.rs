pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ApiKey {
        #[serde(rename = "bearer")]
        #[non_exhaustive]
        Bearer {
            #[serde(default)]
            api_key_id: ApiKeyId,
            #[serde(default)]
            prefix: Prefix,
            #[serde(default)]
            name: Name,
            #[serde(skip_serializing_if = "Option::is_none")]
            pod_id: Option<PodScopeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            inbox_id: Option<InboxScopeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            used_at: Option<UsedAt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            permissions: Option<ApiKeyPermissions>,
            #[serde(default)]
            created_at: CreatedAt,
            #[serde(default)]
            updated_at: UpdatedAt,
            #[serde(skip_serializing_if = "Option::is_none")]
            expires_at: Option<ExpiresAt>,
        },

        #[serde(rename = "public_key")]
        #[non_exhaustive]
        PublicKey {
            #[serde(flatten)]
            data: PublicKeyCredential,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ApiKey {
    pub fn bearer(api_key_id: ApiKeyId, prefix: Prefix, name: Name, created_at: CreatedAt, updated_at: UpdatedAt) -> Self {
        Self::Bearer { api_key_id, prefix, name, pod_id: None, inbox_id: None, used_at: None, permissions: None, created_at, updated_at, expires_at: None }
    }

    pub fn public_key(data: PublicKeyCredential) -> Self {
        Self::PublicKey { data }
    }

    pub fn bearer_with_pod_id(api_key_id: ApiKeyId, prefix: Prefix, name: Name, pod_id: PodScopeId, inbox_id: Option<InboxScopeId>, used_at: Option<UsedAt>, permissions: Option<ApiKeyPermissions>, created_at: CreatedAt, updated_at: UpdatedAt, expires_at: Option<ExpiresAt>) -> Self {
        Self::Bearer { api_key_id, prefix, name, pod_id: Some(pod_id), inbox_id, used_at, permissions, created_at, updated_at, expires_at }
    }

    pub fn bearer_with_inbox_id(api_key_id: ApiKeyId, prefix: Prefix, name: Name, pod_id: Option<PodScopeId>, inbox_id: InboxScopeId, used_at: Option<UsedAt>, permissions: Option<ApiKeyPermissions>, created_at: CreatedAt, updated_at: UpdatedAt, expires_at: Option<ExpiresAt>) -> Self {
        Self::Bearer { api_key_id, prefix, name, pod_id, inbox_id: Some(inbox_id), used_at, permissions, created_at, updated_at, expires_at }
    }

    pub fn bearer_with_used_at(api_key_id: ApiKeyId, prefix: Prefix, name: Name, pod_id: Option<PodScopeId>, inbox_id: Option<InboxScopeId>, used_at: UsedAt, permissions: Option<ApiKeyPermissions>, created_at: CreatedAt, updated_at: UpdatedAt, expires_at: Option<ExpiresAt>) -> Self {
        Self::Bearer { api_key_id, prefix, name, pod_id, inbox_id, used_at: Some(used_at), permissions, created_at, updated_at, expires_at }
    }

    pub fn bearer_with_permissions(api_key_id: ApiKeyId, prefix: Prefix, name: Name, pod_id: Option<PodScopeId>, inbox_id: Option<InboxScopeId>, used_at: Option<UsedAt>, permissions: ApiKeyPermissions, created_at: CreatedAt, updated_at: UpdatedAt, expires_at: Option<ExpiresAt>) -> Self {
        Self::Bearer { api_key_id, prefix, name, pod_id, inbox_id, used_at, permissions: Some(permissions), created_at, updated_at, expires_at }
    }

    pub fn bearer_with_expires_at(api_key_id: ApiKeyId, prefix: Prefix, name: Name, pod_id: Option<PodScopeId>, inbox_id: Option<InboxScopeId>, used_at: Option<UsedAt>, permissions: Option<ApiKeyPermissions>, created_at: CreatedAt, updated_at: UpdatedAt, expires_at: ExpiresAt) -> Self {
        Self::Bearer { api_key_id, prefix, name, pod_id, inbox_id, used_at, permissions, created_at, updated_at, expires_at: Some(expires_at) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
