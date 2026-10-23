pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentArmorReleaseConsoleActor {
}

impl AgentArmorReleaseConsoleActor {
    pub fn builder() -> AgentArmorReleaseConsoleActorBuilder {
        <AgentArmorReleaseConsoleActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorReleaseConsoleActorBuilder {
}

impl AgentArmorReleaseConsoleActorBuilder {

    /// Consumes the builder and constructs a [`AgentArmorReleaseConsoleActor`].
    pub fn build(self) -> Result<AgentArmorReleaseConsoleActor, BuildError> {
        Ok(AgentArmorReleaseConsoleActor {
        })
    }
}
