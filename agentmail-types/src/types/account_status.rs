pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `disabled` refuses every new sign-in for the account until it is
/// re-enabled. Access tokens already issued are not revoked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccountStatus {
    #[serde(rename = "disabled")]
    Disabled,
}
impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Disabled => "disabled",
        };
        write!(f, "{}", s)
    }
}
