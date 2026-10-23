pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the organization is in AgentMail Labs, and whether Labs is offered in this deployment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationLabs {
    /// Whether the organization is in AgentMail Labs.
    #[serde(default)]
    pub enabled: bool,
    /// Time at which the organization joined Labs. `null` when it is not in Labs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<DateTime<FixedOffset>>,
    /// Whether Labs is offered in this deployment. Labs is available in US production (`api.agentmail.to`), not yet in EU production (`api.agentmail.eu`).
    #[serde(default)]
    pub available: bool,
}

impl OrganizationLabs {
    pub fn builder() -> OrganizationLabsBuilder {
        <OrganizationLabsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationLabsBuilder {
    enabled: Option<bool>,
    enabled_at: Option<DateTime<FixedOffset>>,
    available: Option<bool>,
}

impl OrganizationLabsBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn enabled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.enabled_at = Some(value);
        self
    }

    pub fn available(mut self, value: bool) -> Self {
        self.available = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationLabs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](OrganizationLabsBuilder::enabled)
    /// - [`available`](OrganizationLabsBuilder::available)
    pub fn build(self) -> Result<OrganizationLabs, BuildError> {
        Ok(OrganizationLabs {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            enabled_at: self.enabled_at,
            available: self.available.ok_or_else(|| BuildError::missing_field("available"))?,
        })
    }
}
