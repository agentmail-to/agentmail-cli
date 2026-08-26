pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Registered public key material and its server-computed RFC 7638 thumbprint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKeyMaterial {
    pub jwk: PublicJwk,
    /// RFC 7638 SHA-256 JWK thumbprint encoded as unpadded base64url.
    #[serde(default)]
    pub fingerprint: String,
}

impl PublicKeyMaterial {
    pub fn builder() -> PublicKeyMaterialBuilder {
        <PublicKeyMaterialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicKeyMaterialBuilder {
    jwk: Option<PublicJwk>,
    fingerprint: Option<String>,
}

impl PublicKeyMaterialBuilder {
    pub fn jwk(mut self, value: PublicJwk) -> Self {
        self.jwk = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublicKeyMaterial`].
    /// This method will fail if any of the following fields are not set:
    /// - [`jwk`](PublicKeyMaterialBuilder::jwk)
    /// - [`fingerprint`](PublicKeyMaterialBuilder::fingerprint)
    pub fn build(self) -> Result<PublicKeyMaterial, BuildError> {
        Ok(PublicKeyMaterial {
            jwk: self.jwk.ok_or_else(|| BuildError::missing_field("jwk"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
        })
    }
}
