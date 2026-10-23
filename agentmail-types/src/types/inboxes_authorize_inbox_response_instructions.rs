pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The agent's next step. Nothing further is required from the agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InboxesAuthorizeInboxResponseInstructions {
    #[serde(rename = "Authorization complete. Return to browser.")]
    AuthorizationCompleteReturnToBrowser,
}
impl fmt::Display for InboxesAuthorizeInboxResponseInstructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AuthorizationCompleteReturnToBrowser => "Authorization complete. Return to browser.",
        };
        write!(f, "{}", s)
    }
}
