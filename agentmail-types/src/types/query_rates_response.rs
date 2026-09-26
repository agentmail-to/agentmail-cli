pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryRatesResponse(pub HashMap<String, Vec<RatePoint>>);