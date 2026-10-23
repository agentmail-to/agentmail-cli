pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An app an inbox can sign in to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct App {
    #[serde(default)]
    pub app_id: AppId,
    /// Short name accepted in place of `app_id` by Get App, List App Accounts and Connect App. Set by AgentMail on apps in the catalog; renaming the app does not change it. Store `app_id`, the app's permanent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Time at which app was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_url: Option<String>,
    /// Kinds of app, up to 3. Omitted when the app sets none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<AppCategory>>,
    /// Maximum number of accounts your organization may sign up at this app. Omitted when the app sets no limit. 0 means the app has paused new sign-ups; existing accounts keep signing in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_signup_limit: Option<i64>,
}

impl App {
    pub fn builder() -> AppBuilder {
        <AppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppBuilder {
    app_id: Option<AppId>,
    slug: Option<String>,
    name: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    logo_url: Option<String>,
    terms_url: Option<String>,
    privacy_url: Option<String>,
    categories: Option<Vec<AppCategory>>,
    owner_signup_limit: Option<i64>,
}

impl AppBuilder {
    pub fn app_id(mut self, value: AppId) -> Self {
        self.app_id = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn terms_url(mut self, value: impl Into<String>) -> Self {
        self.terms_url = Some(value.into());
        self
    }

    pub fn privacy_url(mut self, value: impl Into<String>) -> Self {
        self.privacy_url = Some(value.into());
        self
    }

    pub fn categories(mut self, value: Vec<AppCategory>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn owner_signup_limit(mut self, value: i64) -> Self {
        self.owner_signup_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`App`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_id`](AppBuilder::app_id)
    pub fn build(self) -> Result<App, BuildError> {
        Ok(App {
            app_id: self.app_id.ok_or_else(|| BuildError::missing_field("app_id"))?,
            slug: self.slug,
            name: self.name,
            updated_at: self.updated_at,
            description: self.description,
            logo_url: self.logo_url,
            terms_url: self.terms_url,
            privacy_url: self.privacy_url,
            categories: self.categories,
            owner_signup_limit: self.owner_signup_limit,
        })
    }
}
