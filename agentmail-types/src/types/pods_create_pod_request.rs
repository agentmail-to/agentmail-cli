pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodsCreatePodRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<PodsName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<PodsClientId>,
}

impl PodsCreatePodRequest {
    pub fn builder() -> PodsCreatePodRequestBuilder {
        <PodsCreatePodRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodsCreatePodRequestBuilder {
    name: Option<PodsName>,
    client_id: Option<PodsClientId>,
}

impl PodsCreatePodRequestBuilder {
    pub fn name(mut self, value: PodsName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn client_id(mut self, value: PodsClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodsCreatePodRequest`].
    pub fn build(self) -> Result<PodsCreatePodRequest, BuildError> {
        Ok(PodsCreatePodRequest {
            name: self.name,
            client_id: self.client_id,
        })
    }
}

