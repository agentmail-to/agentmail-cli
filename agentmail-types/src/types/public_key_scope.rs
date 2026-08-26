pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PublicKeyScope {
        PublicKeyScopeZero(PublicKeyScopeZero),

        PublicKeyScopeOne(PublicKeyScopeOne),

        PublicKeyScopeTwo(PublicKeyScopeTwo),
}

impl PublicKeyScope {
    pub fn is_public_key_scope_zero(&self) -> bool {
        matches!(self, Self::PublicKeyScopeZero(_))
    }

    pub fn is_public_key_scope_one(&self) -> bool {
        matches!(self, Self::PublicKeyScopeOne(_))
    }

    pub fn is_public_key_scope_two(&self) -> bool {
        matches!(self, Self::PublicKeyScopeTwo(_))
    }


    pub fn as_public_key_scope_zero(&self) -> Option<&PublicKeyScopeZero> {
        match self {
                    Self::PublicKeyScopeZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_public_key_scope_zero(self) -> Option<PublicKeyScopeZero> {
        match self {
                    Self::PublicKeyScopeZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_public_key_scope_one(&self) -> Option<&PublicKeyScopeOne> {
        match self {
                    Self::PublicKeyScopeOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_public_key_scope_one(self) -> Option<PublicKeyScopeOne> {
        match self {
                    Self::PublicKeyScopeOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_public_key_scope_two(&self) -> Option<&PublicKeyScopeTwo> {
        match self {
                    Self::PublicKeyScopeTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_public_key_scope_two(self) -> Option<PublicKeyScopeTwo> {
        match self {
                    Self::PublicKeyScopeTwo(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for PublicKeyScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PublicKeyScopeZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PublicKeyScopeOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PublicKeyScopeTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
