pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateApiKeyRequest {
        CreateBearerApiKeyRequest(CreateBearerApiKeyRequest),

        CreatePublicKeyRequest(CreatePublicKeyRequest),
}

impl CreateApiKeyRequest {
    pub fn is_create_bearer_api_key_request(&self) -> bool {
        matches!(self, Self::CreateBearerApiKeyRequest(_))
    }

    pub fn is_create_public_key_request(&self) -> bool {
        matches!(self, Self::CreatePublicKeyRequest(_))
    }


    pub fn as_create_bearer_api_key_request(&self) -> Option<&CreateBearerApiKeyRequest> {
        match self {
                    Self::CreateBearerApiKeyRequest(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_bearer_api_key_request(self) -> Option<CreateBearerApiKeyRequest> {
        match self {
                    Self::CreateBearerApiKeyRequest(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_public_key_request(&self) -> Option<&CreatePublicKeyRequest> {
        match self {
                    Self::CreatePublicKeyRequest(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_public_key_request(self) -> Option<CreatePublicKeyRequest> {
        match self {
                    Self::CreatePublicKeyRequest(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateApiKeyRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateBearerApiKeyRequest(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreatePublicKeyRequest(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
