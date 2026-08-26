pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Identity and scope of the authenticated credential.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Identity {
    pub scope_type: ScopeType,
    /// ID of the most specific scope the credential is bound to.
    /// Equals inbox_id when scope_type is inbox, pod_id when pod, organization_id when organization.
    #[serde(default)]
    pub scope_id: String,
    #[serde(default)]
    pub organization_id: OrganizationId,
    /// ID of the pod the credential is scoped to. Present when scope_type is pod or inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    /// ID of the inbox the credential is scoped to. Present when scope_type is inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// ID of the API key used to authenticate. Absent for JWT and proxy credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<String>,
}

impl Identity {
    pub fn builder() -> IdentityBuilder {
        <IdentityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentityBuilder {
    scope_type: Option<ScopeType>,
    scope_id: Option<String>,
    organization_id: Option<OrganizationId>,
    pod_id: Option<String>,
    inbox_id: Option<String>,
    api_key_id: Option<String>,
}

impl IdentityBuilder {
    pub fn scope_type(mut self, value: ScopeType) -> Self {
        self.scope_type = Some(value);
        self
    }

    pub fn scope_id(mut self, value: impl Into<String>) -> Self {
        self.scope_id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn pod_id(mut self, value: impl Into<String>) -> Self {
        self.pod_id = Some(value.into());
        self
    }

    pub fn inbox_id(mut self, value: impl Into<String>) -> Self {
        self.inbox_id = Some(value.into());
        self
    }

    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Identity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scope_type`](IdentityBuilder::scope_type)
    /// - [`scope_id`](IdentityBuilder::scope_id)
    /// - [`organization_id`](IdentityBuilder::organization_id)
    pub fn build(self) -> Result<Identity, BuildError> {
        Ok(Identity {
            scope_type: self.scope_type.ok_or_else(|| BuildError::missing_field("scope_type"))?,
            scope_id: self.scope_id.ok_or_else(|| BuildError::missing_field("scope_id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            pod_id: self.pod_id,
            inbox_id: self.inbox_id,
            api_key_id: self.api_key_id,
        })
    }
}
