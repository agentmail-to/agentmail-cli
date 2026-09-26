pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RatePoint {
    /// End of the window the point covers; the window is the `window` seconds before it.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Bounced or complained messages divided by messages sent over the
    /// window, as a fraction (0.05 is 5%). Null when nothing was sent in
    /// the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// Messages sent in the window, counted per recipient.
    #[serde(default)]
    pub sent: i64,
}

impl RatePoint {
    pub fn builder() -> RatePointBuilder {
        <RatePointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RatePointBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    rate: Option<f64>,
    sent: Option<i64>,
}

impl RatePointBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn rate(mut self, value: f64) -> Self {
        self.rate = Some(value);
        self
    }

    pub fn sent(mut self, value: i64) -> Self {
        self.sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RatePoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](RatePointBuilder::timestamp)
    /// - [`sent`](RatePointBuilder::sent)
    pub fn build(self) -> Result<RatePoint, BuildError> {
        Ok(RatePoint {
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            rate: self.rate,
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
        })
    }
}
