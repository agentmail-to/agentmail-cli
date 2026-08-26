pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Server-owned credential discriminator. Callers cannot select or update it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PublicKeyCredentialType {
    #[serde(rename = "public_key")]
    PublicKey,
}
impl fmt::Display for PublicKeyCredentialType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PublicKey => "public_key",
        };
        write!(f, "{}", s)
    }
}
