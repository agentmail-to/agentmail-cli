pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageSecurityReviewEventEventType {
    #[serde(rename = "message.security.review")]
    MessageSecurityReview,
}
impl fmt::Display for MessageSecurityReviewEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MessageSecurityReview => "message.security.review",
        };
        write!(f, "{}", s)
    }
}
