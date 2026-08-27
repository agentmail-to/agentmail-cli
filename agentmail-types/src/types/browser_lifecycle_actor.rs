pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BrowserLifecycleActor {
        BrowserLifecycleActorZero(BrowserLifecycleActorZero),

        BrowserLifecycleActorOne(BrowserLifecycleActorOne),
}

impl BrowserLifecycleActor {
    pub fn is_browser_lifecycle_actor_zero(&self) -> bool {
        matches!(self, Self::BrowserLifecycleActorZero(_))
    }

    pub fn is_browser_lifecycle_actor_one(&self) -> bool {
        matches!(self, Self::BrowserLifecycleActorOne(_))
    }


    pub fn as_browser_lifecycle_actor_zero(&self) -> Option<&BrowserLifecycleActorZero> {
        match self {
                    Self::BrowserLifecycleActorZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_browser_lifecycle_actor_zero(self) -> Option<BrowserLifecycleActorZero> {
        match self {
                    Self::BrowserLifecycleActorZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_browser_lifecycle_actor_one(&self) -> Option<&BrowserLifecycleActorOne> {
        match self {
                    Self::BrowserLifecycleActorOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_browser_lifecycle_actor_one(self) -> Option<BrowserLifecycleActorOne> {
        match self {
                    Self::BrowserLifecycleActorOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BrowserLifecycleActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrowserLifecycleActorZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::BrowserLifecycleActorOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
