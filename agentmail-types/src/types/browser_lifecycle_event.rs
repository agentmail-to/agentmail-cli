pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BrowserLifecycleEvent {
        BrowserEnrollmentLifecycleEvent(BrowserEnrollmentLifecycleEvent),

        BrowserConsentLifecycleEvent(BrowserConsentLifecycleEvent),
}

impl BrowserLifecycleEvent {
    pub fn is_browser_enrollment_lifecycle_event(&self) -> bool {
        matches!(self, Self::BrowserEnrollmentLifecycleEvent(_))
    }

    pub fn is_browser_consent_lifecycle_event(&self) -> bool {
        matches!(self, Self::BrowserConsentLifecycleEvent(_))
    }


    pub fn as_browser_enrollment_lifecycle_event(&self) -> Option<&BrowserEnrollmentLifecycleEvent> {
        match self {
                    Self::BrowserEnrollmentLifecycleEvent(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_browser_enrollment_lifecycle_event(self) -> Option<BrowserEnrollmentLifecycleEvent> {
        match self {
                    Self::BrowserEnrollmentLifecycleEvent(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_browser_consent_lifecycle_event(&self) -> Option<&BrowserConsentLifecycleEvent> {
        match self {
                    Self::BrowserConsentLifecycleEvent(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_browser_consent_lifecycle_event(self) -> Option<BrowserConsentLifecycleEvent> {
        match self {
                    Self::BrowserConsentLifecycleEvent(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BrowserLifecycleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrowserEnrollmentLifecycleEvent(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::BrowserConsentLifecycleEvent(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
