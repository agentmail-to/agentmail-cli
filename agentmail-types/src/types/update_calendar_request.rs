pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCalendarRequest {
    /// New default time zone for events created without a `timezone`.
    #[serde(default)]
    pub timezone: IanaTimezone,
}

impl UpdateCalendarRequest {
    pub fn builder() -> UpdateCalendarRequestBuilder {
        <UpdateCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarRequestBuilder {
    timezone: Option<IanaTimezone>,
}

impl UpdateCalendarRequestBuilder {
    pub fn timezone(mut self, value: IanaTimezone) -> Self {
        self.timezone = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timezone`](UpdateCalendarRequestBuilder::timezone)
    pub fn build(self) -> Result<UpdateCalendarRequest, BuildError> {
        Ok(UpdateCalendarRequest {
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
        })
    }
}

