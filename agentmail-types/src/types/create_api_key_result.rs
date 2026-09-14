pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateApiKeyResult {
        CreateApiKeyResponse(CreateApiKeyResponse),

        PublicKeyCredential(PublicKeyCredential),
}

impl CreateApiKeyResult {
    pub fn is_create_api_key_response(&self) -> bool {
        matches!(self, Self::CreateApiKeyResponse(_))
    }

    pub fn is_public_key_credential(&self) -> bool {
        matches!(self, Self::PublicKeyCredential(_))
    }


    pub fn as_create_api_key_response(&self) -> Option<&CreateApiKeyResponse> {
        match self {
                    Self::CreateApiKeyResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_api_key_response(self) -> Option<CreateApiKeyResponse> {
        match self {
                    Self::CreateApiKeyResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_public_key_credential(&self) -> Option<&PublicKeyCredential> {
        match self {
                    Self::PublicKeyCredential(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_public_key_credential(self) -> Option<PublicKeyCredential> {
        match self {
                    Self::PublicKeyCredential(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateApiKeyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateApiKeyResponse(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PublicKeyCredential(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
