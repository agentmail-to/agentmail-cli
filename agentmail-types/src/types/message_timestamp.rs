pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct MessageTimestamp(
    #[serde(deserialize_with = "crate::core::flexible_datetime::offset::deserialize")]
    pub DateTime<FixedOffset>
);