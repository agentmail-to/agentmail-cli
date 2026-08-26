pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A public P-256 JWK. The object accepts exactly `kty`, `crv`, `x`, and `y`.
/// Private key material such as `d`, embedded key IDs, and all other members
/// are rejected. The server also rejects coordinates that are not a point on
/// P-256.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicJwk {
    pub kty: PublicJwkKty,
    pub crv: PublicJwkCrv,
    #[serde(default)]
    pub x: PublicJwkCoordinate,
    #[serde(default)]
    pub y: PublicJwkCoordinate,
}

impl PublicJwk {
    pub fn builder() -> PublicJwkBuilder {
        <PublicJwkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicJwkBuilder {
    kty: Option<PublicJwkKty>,
    crv: Option<PublicJwkCrv>,
    x: Option<PublicJwkCoordinate>,
    y: Option<PublicJwkCoordinate>,
}

impl PublicJwkBuilder {
    pub fn kty(mut self, value: PublicJwkKty) -> Self {
        self.kty = Some(value);
        self
    }

    pub fn crv(mut self, value: PublicJwkCrv) -> Self {
        self.crv = Some(value);
        self
    }

    pub fn x(mut self, value: PublicJwkCoordinate) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: PublicJwkCoordinate) -> Self {
        self.y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicJwk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kty`](PublicJwkBuilder::kty)
    /// - [`crv`](PublicJwkBuilder::crv)
    /// - [`x`](PublicJwkBuilder::x)
    /// - [`y`](PublicJwkBuilder::y)
    pub fn build(self) -> Result<PublicJwk, BuildError> {
        Ok(PublicJwk {
            kty: self.kty.ok_or_else(|| BuildError::missing_field("kty"))?,
            crv: self.crv.ok_or_else(|| BuildError::missing_field("crv"))?,
            x: self.x.ok_or_else(|| BuildError::missing_field("x"))?,
            y: self.y.ok_or_else(|| BuildError::missing_field("y"))?,
        })
    }
}
