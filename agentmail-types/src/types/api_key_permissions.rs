pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Granular permissions for the API key. When ommitted all permissions are granted. Otherwise, only permissions set to true are granted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyPermissions {
    /// Read inbox details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_read: Option<bool>,
    /// Create new inboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_create: Option<bool>,
    /// Update inbox settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_update: Option<bool>,
    /// Delete inboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_delete: Option<bool>,
    /// Read messages. Also required to read threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_read: Option<bool>,
    /// Send messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_send: Option<bool>,
    /// Update message labels. Also required to update threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_update: Option<bool>,
    /// Delete messages. Also required to delete threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_delete: Option<bool>,
    /// Access messages labeled spam.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_spam_read: Option<bool>,
    /// Access messages labeled blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_blocked_read: Option<bool>,
    /// Access messages labeled unauthenticated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_unauthenticated_read: Option<bool>,
    /// Access messages labeled trash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_trash_read: Option<bool>,
    /// Read drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_read: Option<bool>,
    /// Create drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_create: Option<bool>,
    /// Update drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_update: Option<bool>,
    /// Delete drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_delete: Option<bool>,
    /// Send drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_send: Option<bool>,
    /// Read webhook configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_read: Option<bool>,
    /// Create webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_create: Option<bool>,
    /// Update webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_update: Option<bool>,
    /// Delete webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_delete: Option<bool>,
    /// Read domain details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_read: Option<bool>,
    /// Create domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_create: Option<bool>,
    /// Update domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_update: Option<bool>,
    /// Delete domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_delete: Option<bool>,
    /// Read list entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_entry_read: Option<bool>,
    /// Create list entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_entry_create: Option<bool>,
    /// Delete list entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_entry_delete: Option<bool>,
    /// Read metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_read: Option<bool>,
    /// Read API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_read: Option<bool>,
    /// Create API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_create: Option<bool>,
    /// Update API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_update: Option<bool>,
    /// Delete API keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_delete: Option<bool>,
    /// Read pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_read: Option<bool>,
    /// Create pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_create: Option<bool>,
    /// Delete pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_delete: Option<bool>,
}

impl ApiKeyPermissions {
    pub fn builder() -> ApiKeyPermissionsBuilder {
        <ApiKeyPermissionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyPermissionsBuilder {
    inbox_read: Option<bool>,
    inbox_create: Option<bool>,
    inbox_update: Option<bool>,
    inbox_delete: Option<bool>,
    message_read: Option<bool>,
    message_send: Option<bool>,
    message_update: Option<bool>,
    message_delete: Option<bool>,
    label_spam_read: Option<bool>,
    label_blocked_read: Option<bool>,
    label_unauthenticated_read: Option<bool>,
    label_trash_read: Option<bool>,
    draft_read: Option<bool>,
    draft_create: Option<bool>,
    draft_update: Option<bool>,
    draft_delete: Option<bool>,
    draft_send: Option<bool>,
    webhook_read: Option<bool>,
    webhook_create: Option<bool>,
    webhook_update: Option<bool>,
    webhook_delete: Option<bool>,
    domain_read: Option<bool>,
    domain_create: Option<bool>,
    domain_update: Option<bool>,
    domain_delete: Option<bool>,
    list_entry_read: Option<bool>,
    list_entry_create: Option<bool>,
    list_entry_delete: Option<bool>,
    metrics_read: Option<bool>,
    api_key_read: Option<bool>,
    api_key_create: Option<bool>,
    api_key_update: Option<bool>,
    api_key_delete: Option<bool>,
    pod_read: Option<bool>,
    pod_create: Option<bool>,
    pod_delete: Option<bool>,
}

impl ApiKeyPermissionsBuilder {
    pub fn inbox_read(mut self, value: bool) -> Self {
        self.inbox_read = Some(value);
        self
    }

    pub fn inbox_create(mut self, value: bool) -> Self {
        self.inbox_create = Some(value);
        self
    }

    pub fn inbox_update(mut self, value: bool) -> Self {
        self.inbox_update = Some(value);
        self
    }

    pub fn inbox_delete(mut self, value: bool) -> Self {
        self.inbox_delete = Some(value);
        self
    }

    pub fn message_read(mut self, value: bool) -> Self {
        self.message_read = Some(value);
        self
    }

    pub fn message_send(mut self, value: bool) -> Self {
        self.message_send = Some(value);
        self
    }

    pub fn message_update(mut self, value: bool) -> Self {
        self.message_update = Some(value);
        self
    }

    pub fn message_delete(mut self, value: bool) -> Self {
        self.message_delete = Some(value);
        self
    }

    pub fn label_spam_read(mut self, value: bool) -> Self {
        self.label_spam_read = Some(value);
        self
    }

    pub fn label_blocked_read(mut self, value: bool) -> Self {
        self.label_blocked_read = Some(value);
        self
    }

    pub fn label_unauthenticated_read(mut self, value: bool) -> Self {
        self.label_unauthenticated_read = Some(value);
        self
    }

    pub fn label_trash_read(mut self, value: bool) -> Self {
        self.label_trash_read = Some(value);
        self
    }

    pub fn draft_read(mut self, value: bool) -> Self {
        self.draft_read = Some(value);
        self
    }

    pub fn draft_create(mut self, value: bool) -> Self {
        self.draft_create = Some(value);
        self
    }

    pub fn draft_update(mut self, value: bool) -> Self {
        self.draft_update = Some(value);
        self
    }

    pub fn draft_delete(mut self, value: bool) -> Self {
        self.draft_delete = Some(value);
        self
    }

    pub fn draft_send(mut self, value: bool) -> Self {
        self.draft_send = Some(value);
        self
    }

    pub fn webhook_read(mut self, value: bool) -> Self {
        self.webhook_read = Some(value);
        self
    }

    pub fn webhook_create(mut self, value: bool) -> Self {
        self.webhook_create = Some(value);
        self
    }

    pub fn webhook_update(mut self, value: bool) -> Self {
        self.webhook_update = Some(value);
        self
    }

    pub fn webhook_delete(mut self, value: bool) -> Self {
        self.webhook_delete = Some(value);
        self
    }

    pub fn domain_read(mut self, value: bool) -> Self {
        self.domain_read = Some(value);
        self
    }

    pub fn domain_create(mut self, value: bool) -> Self {
        self.domain_create = Some(value);
        self
    }

    pub fn domain_update(mut self, value: bool) -> Self {
        self.domain_update = Some(value);
        self
    }

    pub fn domain_delete(mut self, value: bool) -> Self {
        self.domain_delete = Some(value);
        self
    }

    pub fn list_entry_read(mut self, value: bool) -> Self {
        self.list_entry_read = Some(value);
        self
    }

    pub fn list_entry_create(mut self, value: bool) -> Self {
        self.list_entry_create = Some(value);
        self
    }

    pub fn list_entry_delete(mut self, value: bool) -> Self {
        self.list_entry_delete = Some(value);
        self
    }

    pub fn metrics_read(mut self, value: bool) -> Self {
        self.metrics_read = Some(value);
        self
    }

    pub fn api_key_read(mut self, value: bool) -> Self {
        self.api_key_read = Some(value);
        self
    }

    pub fn api_key_create(mut self, value: bool) -> Self {
        self.api_key_create = Some(value);
        self
    }

    pub fn api_key_update(mut self, value: bool) -> Self {
        self.api_key_update = Some(value);
        self
    }

    pub fn api_key_delete(mut self, value: bool) -> Self {
        self.api_key_delete = Some(value);
        self
    }

    pub fn pod_read(mut self, value: bool) -> Self {
        self.pod_read = Some(value);
        self
    }

    pub fn pod_create(mut self, value: bool) -> Self {
        self.pod_create = Some(value);
        self
    }

    pub fn pod_delete(mut self, value: bool) -> Self {
        self.pod_delete = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyPermissions`].
    pub fn build(self) -> Result<ApiKeyPermissions, BuildError> {
        Ok(ApiKeyPermissions {
            inbox_read: self.inbox_read,
            inbox_create: self.inbox_create,
            inbox_update: self.inbox_update,
            inbox_delete: self.inbox_delete,
            message_read: self.message_read,
            message_send: self.message_send,
            message_update: self.message_update,
            message_delete: self.message_delete,
            label_spam_read: self.label_spam_read,
            label_blocked_read: self.label_blocked_read,
            label_unauthenticated_read: self.label_unauthenticated_read,
            label_trash_read: self.label_trash_read,
            draft_read: self.draft_read,
            draft_create: self.draft_create,
            draft_update: self.draft_update,
            draft_delete: self.draft_delete,
            draft_send: self.draft_send,
            webhook_read: self.webhook_read,
            webhook_create: self.webhook_create,
            webhook_update: self.webhook_update,
            webhook_delete: self.webhook_delete,
            domain_read: self.domain_read,
            domain_create: self.domain_create,
            domain_update: self.domain_update,
            domain_delete: self.domain_delete,
            list_entry_read: self.list_entry_read,
            list_entry_create: self.list_entry_create,
            list_entry_delete: self.list_entry_delete,
            metrics_read: self.metrics_read,
            api_key_read: self.api_key_read,
            api_key_create: self.api_key_create,
            api_key_update: self.api_key_update,
            api_key_delete: self.api_key_delete,
            pod_read: self.pod_read,
            pod_create: self.pod_create,
            pod_delete: self.pod_delete,
        })
    }
}
